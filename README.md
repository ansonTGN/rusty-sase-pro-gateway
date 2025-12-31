# 🛡️ Rusty SASE Pro Gateway

## Secure Access Service Edge (SASE) de Alto Rendimiento

Una plataforma de seguridad de red de código abierto construida con **Rust** para el *data plane* y **Axum/Alpine.js** para el *control plane*. Este proyecto transforma cualquier máquina Linux en un *gateway* de seguridad de borde con filtrado de contenidos de latencia ultrabaja, políticas de recarga en caliente y observabilidad de grado profesional.

| **Estado** | **Versión** | **Licencia** |
| :---: | :---: | :---: |
| ✅ Estable | v1.0.1 | MIT |

---

## 🚀 Características Principales

| Característica | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Arquitectura** | Proxy Asíncrono (`tokio`, `hudsucker`) para alto rendimiento. | Reactivo, Zero-Build (CDN-based Alpine.js + Tailwind CSS). |
| **Observabilidad** | **NSS (Nanolog Streaming Service)**: Logs estructurados en JSON (`tracing`) en `logs/sase.json`. | **SSE (Server-Sent Events)**: Flujo de tráfico en vivo (últimas 10 líneas) al dashboard sin *polling*. |
| **Control de Políticas** | `tokio::sync::RwLock` en la configuración (`AppState`). | **Hot-Reload:** Cambios en la lista negra aplicados al instante. |
| **Admin UI** | Puerto dinámico (`127.0.0.1:0`) seleccionado automáticamente. | Apertura automática del navegador (`opener` crate). |
| **Seguridad** | **Zero Trust Admin:** La API de gestión solo escucha en `127.0.0.1`. | Inspección SSL (Generación de CA con `rcgen`). |

---

## 🏗️ Arquitectura y Flujo de Datos

El *Rusty SASE Pro Gateway* implementa una arquitectura de dos planos:

1.  **Data Plane (Proxy):** Escucha en `0.0.0.0:8080`. Maneja todo el tráfico de la red, aplica el filtro de dominio y emite logs.
2.  **Control Plane (Dashboard):** Escucha en `127.0.0.1:<Puerto_Dinámico>`. Gestiona la configuración, ofrece el Dashboard en tiempo real y sirve el certificado.

### Flujo de Datos SASE

1.  **Cliente** (ej: móvil) dirige el tráfico a `Gateway_IP:8080`.
2.  **Proxy Rust** (`hudsucker`) recibe la petición.
3.  **SASE Handler** bloquea o permite, basándose en la política de `AppState` (`RwLock`).
4.  La actividad se envía simultáneamente al archivo **`logs/sase.json`** y al stream **SSE** del Dashboard.
5.  El **Dashboard** actualiza la tabla de tráfico en tiempo real.

---

## 🛠️ Requisitos e Instalación

### Requisitos

*   **Rust:** Versión 1.70+ (`rustup update`).
*   **Sistema Operativo:** Linux (requerido para los comandos de red con `iptables` y `sudo`).
*   **Permisos:** Se requiere `sudo` para ejecutar el proxy en el puerto `8080` y escribir logs.

### Instalación y Ejecución

1.  **Clonar el repositorio:** (Asume que ya tienes los archivos `Cargo.toml`, `src/main.rs` y la carpeta `static/` con `index.html`.)
2.  **Compilar la versión optimizada (Release):**
    ```bash
    cargo build --release
    ```
3.  **Ejecutar el Gateway (¡Requiere SUDO!):**
    ```bash
    # Se requiere sudo para abrir el puerto 8080 y escribir los logs.
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 Uso del Sistema

Al ejecutar el comando, la terminal mostrará la URL del Dashboard y el Gateway:

```
🛡️  SASE PRO Core Active
Admin UI: http://127.0.0.1:46073  <-- ¡Puerto dinámico!
Proxy: 0.0.0.0:8080
```

### Paso 1: Configuración de Políticas y Dashboard

1.  El navegador se abrirá automáticamente en la URL del **Admin UI** (ej: `http://127.0.0.1:46073`).
2.  En el Dashboard, añade dominios a la lista negra en **Gestión de Políticas** y haz clic en **Guardar**. Los cambios son efectivos inmediatamente.

### Paso 2: Configuración del Cliente (Móvil, PC, etc.)

Para dirigir el tráfico a tu Gateway, debes configurar el proxy en tus dispositivos:

1.  **Busca la IP local de tu máquina Gateway** (ej: `192.168.1.50`).
2.  **En el dispositivo cliente:** Ve a los ajustes de red/Wi-Fi y configura el **Proxy Manual**.
    *   **Host del Proxy:** `<Tu IP local>`
    *   **Puerto del Proxy:** `8080`

### Paso 3: Instalación del Certificado (HTTPS)

Para evitar errores de "conexión no segura" en sitios HTTPS:

1.  En el Dashboard, haz clic en **🛡️ Certificado** y descarga el archivo `ca.crt`.
2.  **Instala `ca.crt`** en el dispositivo cliente como **"Autoridad de Certificación Raíz de Confianza"**. (Este paso es obligatorio para que el proxy funcione con tráfico cifrado).

---
---

# 🇬🇧 ENGLISH README

## 🛡️ Rusty SASE Pro Gateway

## High-Performance Secure Access Service Edge (SASE)

A professional, open-source network security platform built with **Rust** for the data plane and **Axum/Alpine.js** for the control plane. This project transforms any Linux machine into a security gateway featuring ultra-low-latency content filtering, hot-reload policies, and professional-grade observability.

| **Status** | **Version** | **License** |
| :---: | :---: | :---: |
| ✅ Stable | v1.0.1 | MIT |

---

## 🚀 Key Features

| Feature | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Architecture** | Asynchronous Proxy (`tokio`, `hudsucker`) for high concurrency. | Reactive, Zero-Build (CDN-based Alpine.js + Tailwind CSS). |
| **Observability** | **NSS (Nanolog Streaming Service)**: Structured JSON logs (`tracing`) in `logs/sase.json`. | **SSE (Server-Sent Events)**: Live traffic stream (last 10 lines) to the dashboard without polling. |
| **Policy Control** | `tokio::sync::RwLock` for configuration (`AppState`). | **Hot-Reload:** Blacklist changes are applied instantly across the network. |
| **Admin UI** | Dynamic Port (`127.0.0.1:0`) automatically selected upon startup. | Automatic browser launch (`opener` crate). |
| **Security** | **Zero Trust Admin:** Management API listens only on `127.0.0.1`. | SSL Inspection (CA generation with `rcgen`). |

---

## 🛠️ Requirements & Installation

### Prerequisites

*   **Rust:** Version 1.70+ (`rustup update`).
*   **Operating System:** Linux (required for `iptables` and `sudo` network commands).
*   **Permissions:** `sudo` is required to run the proxy on port `8080` and write logs.

### Installation and Execution

1.  **Clone the repository:** (Assumes you have `Cargo.toml`, `src/main.rs`, and the `static/` folder with `index.html`.)
2.  **Build the optimized (Release) version:**
    ```bash
    cargo build --release
    ```
3.  **Run the Gateway (Requires SUDO!):**
    ```bash
    # SUDO is required to open port 8080 and write logs.
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 System Usage

Upon execution, the terminal will display the Dashboard URL and the fixed Gateway port:

```
🛡️  SASE PRO Core Active
Admin UI: http://127.0.0.1:46073  <-- Dynamic Port!
Proxy: 0.0.0.0:8080
```

### Step 1: Policy Configuration

1.  The browser will automatically open the **Admin UI** URL (e.g., `http://127.0.0.1:46073`).
2.  In the Dashboard, add domains to the blacklist under **Policy Management** and click **Save**. Changes are instantaneous.

### Step 2: Client Configuration (Mobile, PC, etc.)

To direct traffic to your Gateway, you must set the proxy on your client devices:

1.  **Find the local IP of your Gateway machine** (e.g., `192.168.1.50`).
2.  **On the client device:** Go to the network/Wi-Fi settings and set the **Manual Proxy**.
    *   **Proxy Host:** `<Your Local IP>` (e.g., `192.168.1.50`)
    *   **Proxy Port:** `8080`

### Step 3: Certificate Installation (HTTPS)

To view HTTPS traffic and avoid "insecure connection" errors:

1.  On the Dashboard, click **🛡️ Certificado** and download the `ca.crt` file.
2.  **Install `ca.crt`** on the client device as a **"Trusted Root Certification Authority."** (This step is mandatory for encrypted traffic filtering).

---
---

#  কাতালান README

## 🛡️ Rusty SASE Pro Gateway

## Passarel·la (Gateway) de Seguretat d'Alt Rendiment (SASE)

Una plataforma de seguretat de xarxa professional i de codi obert construïda amb **Rust** per al *data plane* i **Axum/Alpine.js** per al *control plane*. Aquest projecte transforma qualsevol màquina Linux en una passarel·la de seguretat amb filtratge de continguts de latència ultra baixa, polítiques de recàrrega en calent i observabilitat de grau professional.

| **Estat** | **Versió** | **Llicència** |
| :---: | :---: | :---: |
| ✅ Estable | v1.0.1 | MIT |

---

## 🚀 Característiques Clau

| Característica | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Arquitectura** | Proxy Asíncron (`tokio`, `hudsucker`) per a alta concurrència. | Reactiu, Zero-Build (Alpine.js + Tailwind CSS via CDN). |
| **Observabilitat** | **NSS (Nanolog Streaming Service)**: Registres estructurats JSON (`tracing`) a `logs/sase.json`. | **SSE (Server-Sent Events)**: Flux de trànsit en temps real (últimes 10 línies) al panell sense *polling*. |
| **Control de Polítiques** | `tokio::sync::RwLock` per a la configuració (`AppState`). | **Recàrrega en calent (Hot-Reload):** Els canvis a la llista negra s'apliquen instantàniament. |
| **Admin UI** | Port dinàmic (`127.0.0.1:0`) seleccionat automàticament. | Obertura automàtica del navegador (`opener` crate). |
| **Seguretat** | **Zero Trust Admin:** L'API de gestió només escolta a `127.0.0.1`. | Inspecció SSL (Generació de CA amb `rcgen`). |

---

## 🛠️ Requisits i Instal·lació

### Requisits

*   **Rust:** Versió 1.70+ (`rustup update`).
*   **Sistema Operatiu:** Linux (necessari per a les ordres de xarxa `iptables` i `sudo`).
*   **Permisos:** Es requereix `sudo` per executar el proxy al port `8080` i escriure els registres.

### Instal·lació i Execució

1.  **Clonar el repositori:** (Assumim que ja teniu els fitxers `Cargo.toml`, `src/main.rs` i la carpeta `static/` amb `index.html`.)
2.  **Compilar la versió optimitzada (Release):**
    ```bash
    cargo build --release
    ```
3.  **Executar la Passarel·la (Requereix SUDO!):**
    ```bash
    # Es requereix sudo per obrir el port 8080 i escriure els registres.
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 Ús del Sistema

En executar l'ordre, el terminal mostrarà l'URL del Panell i la Passarel·la:

```
🛡️  SASE PRO Core Active
Admin UI: http://127.0.0.1:46073  <-- Port dinàmic!
Proxy: 0.0.0.0:8080
```

### Pas 1: Configuració de Polítiques

1.  El navegador s'obrirà automàticament a l'URL de l'**Admin UI** (ex: `http://127.0.0.1:46073`).
2.  Al Panell, afegiu dominis a la llista negra a **Gestió de Polítiques** i feu clic a **Guardar**. Els canvis són instantanis.

### Pas 2: Configuració del Client (Mòbil, PC, etc.)

Per dirigir el trànsit a la vostra Passarel·la, heu de configurar el proxy als vostres dispositius:

1.  **Trobeu la IP local de la vostra màquina Passarel·la** (ex: `192.168.1.50`).
2.  **Al dispositiu client:** Aneu a la configuració de xarxa/Wi-Fi i configureu el **Proxy Manual**.
    *   **Host del Proxy:** `<La Vostra IP Local>`
    *   **Port del Proxy:** `8080`

### Pas 3: Instal·lació del Certificat (HTTPS)

Per evitar errors de "connexió no segura" en llocs HTTPS:

1.  Al Panell, feu clic a **🛡️ Certificat** i descarregueu el fitxer `ca.crt`.
2.  **Instal·leu `ca.crt`** al dispositiu client com a **"Autoritat de Certificació Arrel de Confiança"**. (Aquest pas és obligatori per al trànsit xifrat).