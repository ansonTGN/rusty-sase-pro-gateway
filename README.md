# 🛡️ Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) de Alto Rendimiento

**Rusty SASE Pro Gateway** es una implementación *minimal-core* de un *gateway* de seguridad de borde, diseñada para ejecutar el filtrado de tráfico a velocidad nativa. Utilizando el runtime asíncrono de **Rust** (`Tokio`) y una arquitectura de planos de control y datos desacoplados, ofrece observabilidad de nivel profesional y gestión de políticas de latencia ultrabaja.

| **Estado** | **Versión** | **Licencia** | **Autor** |
| :---: | :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | [MIT](LICENSE) | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🚀 Características Clave

| Característica | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Arquitectura** | Proxy Asíncrono (`tokio`, `hudsucker`) para alto rendimiento. | Reactivo, Zero-Build (CDN-based Alpine.js + Tailwind CSS). |
| **Observabilidad NSS** | Logs **estructurados en JSON** (`tracing`) para integración SIEM/SOAR. | **SSE (Server-Sent Events)**: Flujo de tráfico en vivo (últimas 10 líneas) sin *polling*. |
| **Políticas** | `tokio::sync::RwLock` | **Hot-Reload:** Cambios en la lista negra aplicados **instantáneamente**. |
| **Seguridad** | **Zero Trust Admin** (`127.0.0.1`) y Inspección TLS (Generación de CA con `rcgen`). | Interfaz profesional con filtro de logs por IP/Dominio/User-Agent. |

---

## 🔬 Arquitectura Técnica Detallada

El diseño desacopla el *Data Plane* (DP) del *Control Plane* (CP) para garantizar una latencia de filtrado predecible.

### **Data Plane (DP) - Rust Core**

*   **Rendimiento:** Utiliza **Rust** y el runtime **Tokio** para evitar el *jitter* del GC y asegurar latencias de milisegundos.
*   **Aseguramiento Atómico:** La configuración se almacena en `RwLock<AppState>`, garantizando que el *Data Plane* nunca se bloquee, ya que miles de conexiones pueden leer (`read().await`) simultáneamente, mientras que las actualizaciones de políticas (`write().await`) son ultrarrápidas y atómicas.
*   **Trazabilidad:** Los logs NSS capturan metadatos críticos (`src_ip`, `user_agent`) y se escriben en `logs/sase.json` para análisis forense externo.

### **Control Plane (CP) - Axum/SSE**

*   **Acceso Seguro:** El CP escucha en un puerto dinámico de **`127.0.0.1`** (localhost), adhiriéndose estrictamente a un modelo de administración Zero Trust.
*   **UX:** El Dashboard de gestión se lanza automáticamente al iniciar el programa (`opener`) y utiliza **SSE** para la visualización de logs en tiempo real.

---

## 🛠️ Requisitos e Instalación

### Estructura del Proyecto

Asegúrese de que su proyecto tenga la siguiente estructura:

```
rusty-sase-pro/
├── Cargo.toml
├── src/
│   └── main.rs
└── static/
    └── index.html
```

### 1. Dependencias (`Cargo.toml`)

Este archivo define el entorno de ejecución.

```toml
[package]
name = "rusty-sase-pro"
version = "1.0.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-stream = { version = "0.1", features = ["sync"] } # Necesario para SSE
hudsucker = "0.21"
rcgen = "0.11"
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
tracing-appender = "0.2"
chrono = "0.4"
futures-util = { version = "0.3", features = ["io"] }
tower-http = { version = "0.5", features = ["fs"] }
rustls-pemfile = "1.0"
async-trait = "0.1"
opener = "0.7" # Para abrir el navegador
```

### 2. Pasos de Despliegue

1.  **Compilar la versión optimizada (Recomendado):**
    ```bash
    cargo build --release
    ```
2.  **Ejecutar el Gateway (¡Requiere SUDO!):**
    ```bash
    # Se requiere sudo para la vinculación del puerto 8080 y permisos de log.
    sudo ./target/release/rusty-sase-pro
    ```

> **NOTA:** La terminal mostrará el puerto dinámico. El navegador se abrirá automáticamente.

## 🌐 Guía de Uso del Sistema

### Paso 1: Configuración de Políticas y Dashboard

1.  **Dashboard:** El navegador se abrirá automáticamente en `http://127.0.0.1:<Puerto_Dinámico>`.
2.  **Filtrado de Logs:** Utilice el menú desplegable y el campo de texto para filtrar los eventos por **Dominio, IP Origen, Método o User-Agent**.
3.  **Detalles:** Haga clic en cualquier fila de la tabla para ver todos los metadatos capturados (incluido el User-Agent).

### Paso 2: Configuración del Proxy Cliente

Para que el filtrado funcione, debe redirigir el tráfico al proxy:

*   **Gateway IP:** La IP de red local de su máquina (ej: `192.168.1.50`).
*   **Proxy Puerto:** `8080`

**Configuración en Clientes:** Configure el **Proxy Manual** de su navegador o dispositivo a **`<Gateway IP>:8080`**.

### Paso 3: Instalación del Certificado (HTTPS)

Para evitar errores de seguridad en el tráfico cifrado (HTTPS):

1.  En el Dashboard, haga clic en **🛡️ Certificado CA** para descargar `ca.crt`.
2.  Instale `ca.crt` en su dispositivo como una **"Autoridad de Certificación Raíz de Confianza"**.

---

---

# 🇬🇧 ENGLISH README

## 🛡️ Rusty SASE Pro Gateway

## High-Performance Zero Trust SASE (Secure Access Service Edge) Platform

**Rusty SASE Pro Gateway** is a *minimal-core* implementation of an edge security gateway, designed to perform traffic filtering at native speed. Leveraging **Rust**'s asynchronous runtime (`Tokio`) and a decoupled control and data plane architecture, it offers professional-grade observability and ultra-low-latency policy management.

| **Status** | **Version** | **License** | **Author** |
| :---: | :---: | :---: | :---: |
| ✅ Stable | v1.0.2 | [MIT](LICENSE) | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🚀 Key Features

| Feature | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Architecture** | Asynchronous Proxy (`tokio`, `hudsucker`) for high concurrency. | Reactive, Zero-Build (CDN-based Alpine.js + Tailwind CSS). |
| **NSS Observability** | **Structured JSON logs** (`tracing`) for SIEM/SOAR integration. | **SSE (Server-Sent Events)**: Live traffic stream (last 10 lines) without polling. |
| **Policies** | `tokio::sync::RwLock` | **Hot-Reload:** Blacklist changes are applied **instantly**. |
| **Security** | **Zero Trust Admin** (`127.0.0.1`) and TLS Inspection (CA generation with `rcgen`). | Professional UI with log filtering by IP/Domain/User-Agent. |

---

## 🔬 Detailed Technical Architecture (Cybersecurity Ph.D. Level)

The project adheres to the principle of separation of concerns through two decoupled planes, prioritizing data plane performance over management latency.

### **Data Plane (DP) - Rust Core**

*   **Performance:** Uses **Rust** and the **Tokio** runtime to avoid GC *jitter* and ensure ultra-low, predictable latencies, critical for real-time network traffic.
*   **Atomic Assurance:** Configuration is held in `RwLock<AppState>`, ensuring the Data Plane is never blocked, as thousands of connections can read (`read().await`) simultaneously while policy updates (`write().await`) are instantaneous and atomic.
*   **Traceability:** NSS logs capture critical metadata (`src_ip`, `domain`, `user_agent`) and are written to `logs/sase.json` for external forensic analysis.

### **Control Plane (CP) - Axum/SSE**

*   **Secure Access:** The CP listens on a dynamic port on **`127.0.0.1`** (localhost), strictly adhering to a Zero Trust administration model.
*   **UX:** The management Dashboard is launched automatically upon program start (`opener`) and uses **SSE** for real-time log visualization.

---

## 🛠️ Requirements and Deployment

### Project Structure

Ensure your project has the following structure:

```
rusty-sase-pro/
├── Cargo.toml
├── src/
│   └── main.rs
└── static/
    └── index.html
```

### 1. Dependencies (`Cargo.toml`)

This file defines the execution environment.

```toml
[package]
name = "rusty-sase-pro"
version = "1.0.0"
edition = "2021"
# ... (see main Spanish section for full Cargo.toml)
```

### 2. Deployment Steps

1.  **Compile the optimized binary (Recommended):**
    ```bash
    cargo build --release
    ```
2.  **Run the Gateway (Requires SUDO!):**
    ```bash
    # SUDO is required for port 8080 binding and log permissions.
    sudo ./target/release/rusty-sase-pro
    ```

> **NOTE:** The terminal will display the dynamic port. The browser will open automatically.

## 🌐 System Usage Guide

Upon execution, the browser will automatically open the Admin UI, and the proxy will become active.

### 1. Policy and Dashboard Configuration

1.  **Dashboard:** The browser will open automatically to `Admin UI: http://127.0.0.1:<Dynamic_Port>`.
2.  **Log Filtering:** Use the dropdown menu and text field to filter events by **Domain, Source IP, Method, or User-Agent**.
3.  **Details:** Click any row in the table to view all captured metadata (including the User-Agent) in the detail modal.

### 2. Client Proxy Configuration

To make filtering work, you must redirect client traffic to the proxy:

*   **Gateway IP:** The local network IP of your machine (e.g., `192.168.1.50`).
*   **Proxy Port:** `8080`

**Client Setup:** Configure the **Manual Proxy** of your browser or device to **`<Gateway IP>:8080`**.

### 3. Certificate Installation (HTTPS)

To avoid security errors on encrypted traffic (HTTPS):

1.  In the Dashboard, click **🛡️ Certificado CA** to download `ca.crt`.
2.  Install `ca.crt` on your client device as a **"Trusted Root Certification Authority."**

---

---

# 🇨🇦 CATALAN README

## 🛡️ Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) d'Alt Rendiment

**Rusty SASE Pro Gateway** és una implementació *minimal-core* d'una passarel·la de seguretat de vora, dissenyada per executar el filtratge de trànsit a velocitat nativa. Utilitzant el *runtime* asíncron de **Rust** (`Tokio`) i una arquitectura de plans de control i dades desacoblats, ofereix observabilitat de nivell professional i gestió de polítiques de latència ultra baixa.

| **Estat** | **Versió** | **Llicència** | **Autor** |
| :---: | :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | [MIT](LICENSE) | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🚀 Característiques Clau

| Característica | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Arquitectura** | Proxy Asíncron (`tokio`, `hudsucker`) per a alta concurrència. | Reactiu, Zero-Build (Alpine.js + Tailwind CSS via CDN). |
| **Observabilitat NSS** | Registres **estructurats en JSON** (`tracing`) per a integració SIEM/SOAR. | **SSE (Server-Sent Events)**: Flux de trànsit en temps real (últimes 10 línies) sense *polling*. |
| **Polítiques** | `tokio::sync::RwLock` | **Recàrrega en calent (Hot-Reload):** Els canvis a la llista negra s'apliquen **instantàniament**. |
| **Seguretat** | **Zero Trust Admin** (`127.0.0.1`) i Inspecció TLS (Generació de CA amb `rcgen`). | Interfície professional amb filtre de registres per IP/Domini/User-Agent. |

---

## 🔬 Arquitectura Tècnica Detallada (Nivell Dr. en Ciberseguretat)

El projecte s'adhereix al principi de separació de preocupacions a través de dos plans desacoblats, prioritzant el rendiment del *data plane* sobre la latència de gestió.

### **Data Plane (DP) - Rust Core**

*   **Rendiment:** Utilitza **Rust** i el *runtime* **Tokio** per evitar el *jitter* del GC i garantir latències ultrabaixes i predictibles, crucials per al trànsit de xarxa en temps real.
*   **Assegurament Atòmic:** La configuració es manté a `RwLock<AppState>`, assegurant que el *Data Plane* mai es bloquegi, ja que milers de connexions poden llegir (`read().await`) simultàniament mentre que les actualitzacions de polítiques (`write().await`) són instantànies i atòmiques.
*   **Traçabilitat:** Els registres NSS capturen metadades crítiques (`src_ip`, `domain`, `user_agent`) i s'escriuen a `logs/sase.json` per a anàlisi forense externa.

### **Control Plane (CP) - Axum/SSE**

*   **Accés Segur:** El CP escolta en un port dinàmic a **`127.0.0.1`** (localhost), adherint-se estrictament a un model d'administració Zero Trust.
*   **UX:** El Panell de gestió es llança automàticament en iniciar el programa (`opener`) i utilitza **SSE** per a la visualització de registres en temps real.

---

## 🛠️ Requisits i Desplegament

### Estructura del Projecte

Assegureu-vos que el vostre projecte tingui la següent estructura:

```
rusty-sase-pro/
├── Cargo.toml
├── src/
│   └── main.rs
└── static/
    └── index.html
```

### 1. Dependències (`Cargo.toml`)

Aquest fitxer defineix l'entorn d'execució.

```toml
[package]
name = "rusty-sase-pro"
version = "1.0.0"
edition = "2021"
# ... (vegeu la secció principal en espanyol per al Cargo.toml complet)
```

### 2. Passos de Desplegament

1.  **Compilar el binari optimitzat (Recomanat):**
    ```bash
    cargo build --release
    ```
2.  **Executar la Passarel·la (Requereix SUDO!):**
    ```bash
    # Es requereix sudo per a la vinculació del port 8080 i permisos de registre.
    sudo ./target/release/rusty-sase-pro
    ```

> **NOTA:** El terminal mostrarà el port dinàmic. El navegador s'obrirà automàticament.

## 🌐 Guia d'Ús del Sistema

En executar l'ordre, el navegador s'obrirà automàticament a l'Admin UI i el proxy esdevindrà actiu.

### 1. Configuració de Polítiques i Panell

1.  **Panell:** El navegador s'obrirà automàticament a `Admin UI: http://127.0.0.1:<Port_Dinàmic>`.
2.  **Filtrat de Registres:** Utilitzeu el menú desplegable i el camp de text per filtrar els esdeveniments per **Domini, IP Origen, Mètode o User-Agent**.
3.  **Detalls:** Feu clic a qualsevol fila de la taula per veure totes les metadades capturades (inclòs el User-Agent) al modal de detall.

### 2. Configuració del Proxy Client

Perquè el filtratge funcioni, heu de redirigir el trànsit dels clients al proxy:

*   **IP de la Passarel·la:** La IP de xarxa local de la vostra màquina (ex: `192.168.1.50`).
*   **Port del Proxy:** `8080`

**Configuració al Client:** Configureu el **Proxy Manual** del vostre navegador o dispositiu a **`<IP de la Passarel·la>:8080`**.

### 3. Instal·lació del Certificat (HTTPS)

Per evitar errors de seguretat en el trànsit xifrat (HTTPS):

1.  Al Panell, feu clic a **🛡️ Certificado CA** per descarregar `ca.crt`.
2.  Instal·leu `ca.crt` al vostre dispositiu com a **"Autoritat de Certificació Arrel de Confiança"**.