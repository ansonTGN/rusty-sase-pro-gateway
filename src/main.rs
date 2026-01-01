use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post}, 
    Json, Router,
    http::StatusCode,
};
use hudsucker::{
    certificate_authority::RcgenAuthority,
    hyper::{Body, Request, Response},
    Proxy, HttpHandler, HttpContext, RequestOrResponse,
    rustls::{PrivateKey, Certificate}, 
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, fs, convert::Infallible}; 
use tokio::sync::{broadcast, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use async_trait::async_trait;
use futures_util::FutureExt;
use rcgen::{CertificateParams, KeyPair, PKCS_ECDSA_P256_SHA256}; 
use chrono;
use tokio::net::TcpListener;

// --- MODELO DE DATOS ACTUALIZADO: Añadido user_agent ---
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    blocked_domains: Vec<String>,
    stats_blocked_today: u64,
}

#[derive(Debug, Serialize, Clone)]
struct LogEntry {
    timestamp: String,
    src_ip: String,
    domain: String,
    action: String,
    method: String,
    url_path: String,
    user_agent: Option<String>, // NUEVO: User-Agent
}

struct AppState {
    config: RwLock<AppConfig>,
    log_tx: broadcast::Sender<LogEntry>,
}

// --- PROXY HANDLER (DATA PLANE) ---
#[derive(Clone)]
struct SaseHandler {
    state: Arc<AppState>,
}

#[async_trait]
impl HttpHandler for SaseHandler {
    async fn handle_request(&mut self, ctx: &HttpContext, req: Request<Body>) -> RequestOrResponse {
        let host = req.uri().host().unwrap_or("unknown").to_string();
        let src_ip = ctx.client_addr.ip().to_string();
        let mut config = self.state.config.write().await;
        
        let blocked = config.blocked_domains.iter().any(|d| host.contains(d));
        let action = if blocked { "BLOCK" } else { "ALLOW" };
        
        let method = req.method().to_string();
        let url_path = req.uri().path().to_string();

        // --- MEJORA 1: Captura del User-Agent ---
        let user_agent = req.headers()
            .get(hudsucker::hyper::header::USER_AGENT)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        // --- FIN MEJORA 1 ---

        let entry = LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            src_ip: src_ip.clone(),
            domain: host.clone(),
            action: action.to_string(),
            method: method.clone(),
            url_path: url_path.clone(),
            user_agent: user_agent.clone(), // NUEVO
        };

        // Enviar log al stream en vivo y al sistema de archivos (vía tracing)
        let _ = self.state.log_tx.send(entry.clone());
        tracing::info!(
            target: "traffic",
            src_ip = %src_ip,
            domain = %host,
            action = %action,
            method = %method,
            path = %url_path,
            user_agent = ?user_agent // NUEVO en logs
        );

        if blocked {
            config.stats_blocked_today += 1;
            return RequestOrResponse::Response(
                Response::builder().status(403).body(Body::from("Blocked by SASE Pro")).unwrap()
            );
        }

        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> { res }
}

// --- ADMIN API (CONTROL PLANE) ---
async fn update_policy(State(state): State<Arc<AppState>>, Json(new_conf): Json<AppConfig>) -> impl axum::response::IntoResponse {
    let mut conf = state.config.write().await;
    *conf = new_conf;
    // CORRECCIÓN: Especificar el tipo Json<String>
    (StatusCode::OK, Json("Success".to_string())) 
}

async fn get_config(State(state): State<Arc<AppState>>) -> Json<AppConfig> {
    Json(state.config.read().await.clone())
}

async fn log_stream(State(state): State<Arc<AppState>>) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(state.log_tx.subscribe())
        .filter_map(|res| res.ok())
        .map(|entry| {
            Event::default().json_data(entry).unwrap_or_else(|_| Event::default().data("Log Corrupto"))
        })
        .map(Ok); 
        
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

// --- BOOTSTRAP ---
#[tokio::main]
async fn main() {
    // 1. Logs Rotativos en JSON (NSS - Nanolog Streaming Service)
    let file_appender = tracing_appender::rolling::daily("logs", "sase.json");
    tracing_subscriber::fmt()
        .json()
        .with_writer(file_appender)
        .init();

    let (log_tx, _) = broadcast::channel(100);
    let state = Arc::new(AppState {
        config: RwLock::new(AppConfig { blocked_domains: vec!["tiktok.com".into()], stats_blocked_today: 0 }),
        log_tx,
    });

    // 2. Servidor Admin (Localhost Dynamic Port)
    let admin_app = Router::new()
        .route("/api/config", get(get_config).post(update_policy))
        .route("/api/logs/stream", get(log_stream))
        .fallback_service(tower_http::services::ServeDir::new("static"))
        .with_state(state.clone());

    // Usar puerto 0 para elegir uno libre
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let admin_port = listener.local_addr().unwrap().port();
    let admin_url = format!("http://127.0.0.1:{}", admin_port);


    // 3. Proxy Engine y Certificados SSL (Generación de CA)
    let ca = {
        let mut params = CertificateParams::default();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Constrained(0));
        params.distinguished_name = rcgen::DistinguishedName::new();
        params.distinguished_name.push(rcgen::DnType::CommonName, "SASE Root CA");
        
        let key = KeyPair::generate(&PKCS_ECDSA_P256_SHA256).unwrap();
        
        // Obtener el DER de la clave AHORA (antes de moverla)
        let private_key_der = key.serialize_der(); 
        
        // Pasar la propiedad de la clave al params.
        params.key_pair = Some(key); 
        
        let cert = rcgen::Certificate::from_params(params).unwrap(); 
        
        let cert_pem = cert.serialize_pem().unwrap();
        
        fs::create_dir_all("static/certs").ok();
        fs::write("static/certs/ca.crt", &cert_pem).ok(); 
        
        // Obtener el certificado en formato DER (para el wrapper de hudsucker)
        let cert_der = cert.serialize_der().unwrap();

        RcgenAuthority::new(
            PrivateKey(private_key_der), 
            Certificate(cert_der), 
            1000
        ).unwrap()
    };

    println!("🛡️  SASE PRO Core Active");
    println!("Admin UI: {}", admin_url);
    println!("Proxy: 0.0.0.0:8080");

    // LANZAMIENTO AUTOMÁTICO DEL NAVEGADOR (Frontend)
    if let Err(e) = opener::open(&admin_url) {
        eprintln!("ADVERTENCIA: Fallo al abrir el navegador en {}: {}", admin_url, e);
    }

    tokio::select! {
        // Servidor Admin (Control Plane) - Usa el listener con el puerto libre
        _ = axum::serve(listener, admin_app) => {},
        
        // Proxy Engine (Data Plane) - Puerto fijo 8080
        _ = Proxy::builder()
            .with_addr(SocketAddr::from(([0,0,0,0], 8080)))
            .with_rustls_client()
            .with_ca(ca)
            .with_http_handler(SaseHandler { state })
            .build()
            .start(tokio::signal::ctrl_c().map(|_| ()))
             => {}, 
    }
}