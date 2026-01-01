# 🛡️ Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) de Alto Rendimiento

Una arquitectura de seguridad de red de vanguardia implementada en **Rust**, diseñada para un rendimiento de latencia ultrabaja. Rusty SASE Pro Gateway desacopla el **Plano de Datos (Data Plane)** de seguridad del **Plano de Control (Control Plane)** de gestión, ofreciendo Observabilidad NSS y recarga de políticas atómica (**Hot-Reload**).

[![GitHub license](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/yourusername/rusty-sase-pro-gateway)
[![Rust Version](https://img.shields.io/badge/Rust-1.70+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)

| **Tecnologías Core** | **Frontend/UX** | **Patrón de Arquitectura** |
| :--- | :--- | :--- |
| Rust (`Tokio`, `hudsucker`, `rcgen`) | Alpine.js, Tailwind CSS, SSE (Server-Sent Events) | SASE, Zero Trust Admin, Micro-Kernel Networking |

| **Estado** | **Versión** | **Autor** |
| :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🌎 Idiomas

[🇪🇸 Español](#️-rusty-sase-pro-gateway) • [🇬🇧 English](#-rusty-sase-pro-gateway) • [Català](#-rusty-sase-pro-gateway)

---

## 🔬 Arquitectura Técnica Detallada

El diseño del Gateway se centra en la **disponibilidad del servicio (HA)** y la **integridad de las políticas**.

### 1. Data Plane (DP): Lógica de Filtrado de Latencia Ultra Baja

El DP está impulsado por el runtime asíncrono **Tokio** y el framework de proxy `hudsucker`.

| Mecanismo | Implementación Técnica | Racional de Ciberseguridad |
| :--- | :--- | :--- |
| **Rendimiento** | Rust Core (`tokio`) | Elimina el *jitter* de las máquinas virtuales (Java/Go GC) para garantizar una latencia de filtrado predecible, esencial para la experiencia del usuario y la respuesta de red. |
| **Inspección L7** | `rcgen` (Generación de CA/Certificados) | Implementación de **SSL Inspection**. El Gateway intercepta el *handshake* TLS, firma dinámicamente certificados con su CA raíz generada *in-memory* y permite la aplicación de filtros de dominio en tráfico cifrado. |
| **Logs Forenses (NSS)** | `tracing-subscriber::json()` | Generación de logs de tráfico **estructurados en JSON** a través de `tracing`. Cada evento incluye `src_ip`, `user_agent`, `domain` y `action`, lo que garantiza una trazabilidad de nivel forense, lista para la ingesta en plataformas SIEM. |

### 2. Control Plane (CP): Configuración Atómica y Zero Trust

El CP gestiona la configuración de forma segura y proporciona el *dashboard* de observabilidad.

| Mecanismo | Implementación Técnica | Racional de Ciberseguridad |
| :--- | :--- | :--- |
| **Recarga de Políticas** | `tokio::sync::RwLock<AppState>` | **Hot-Reload Atómico**. El DP mantiene un bloqueo de lectura (`read().await`) constante para alta concurrencia. La interfaz de administración realiza un bloqueo de escritura (`write().await`) **instantáneo** para aplicar cambios de política, garantizando la consistencia y la alta disponibilidad del servicio. |
| **Seguridad de Acceso** | `TcpListener::bind("127.0.0.1:0")` | Implementación de **Zero Trust Admin**. La API de gestión solo es accesible desde el localhost, aislando la superficie de ataque de gestión. El puerto dinámico evita conflictos de `AddrInUse`. |
| **Observabilidad UX** | SSE (Server-Sent Events) | Protocolo de *push* de datos para el *Traffic Feed* en el Dashboard. Más eficiente que el *polling* para la visualización de logs en vivo. |

---

## 🛠️ Requisitos e Instalación

### Requisitos

*   **Rust:** Versión 1.70+
*   **Sistema Operativo:** Linux (requerido para los comandos de red).
*   **Privilegios:** Se requiere `sudo` para la ejecución.

### Pasos de Despliegue

1.  **Clonar el Repositorio (o estructurar los archivos):**

    ```bash
    git clone https://github.com/yourusername/rusty-sase-pro-gateway
    cd rusty-sase-pro-gateway
    ```

2.  **Compilar el binario optimizado:**

    ```bash
    cargo build --release
    ```

3.  **Ejecutar el Gateway (¡Requiere SUDO!):**

    ```bash
    # Se recomienda el modo release para el rendimiento.
    sudo ./target/release/rusty-sase-pro
    ```

> **NOTA:** Al arrancar, el navegador se abrirá automáticamente en el puerto dinámico asignado a la **Admin UI**.

## 🌐 Guía de Uso del Sistema

### Paso 1: Configuración de Políticas y Dashboard

1.  **Dashboard:** Se abre automáticamente en `Admin UI: http://127.0.0.1:<Puerto_Dinámico>`.
2.  **Gestión de Políticas:** Añada dominios en la lista negra (Hot-Reload instantáneo al guardar).
3.  **Análisis:** Utilice el menú desplegable y la barra de búsqueda para filtrar el Traffic Feed por **IP Origen, Dominio, Método o User-Agent**. Haga clic en una fila para ver el detalle completo.

### Paso 2: Configuración del Proxy Cliente

Para que el filtrado se aplique, debe dirigir el tráfico de sus dispositivos al Gateway:

*   **Gateway IP:** `<IP Local de su máquina>` (ej: `192.168.1.50`).
*   **Proxy Puerto:** `8080` (Puerto fijo del Data Plane).

### Paso 3: Instalación del Certificado (HTTPS)

**Obligatorio** para el tráfico cifrado:

1.  Descargue **`ca.crt`** desde el Dashboard (enlace 🛡️ Certificado CA).
2.  Instale este archivo en su dispositivo cliente como una **"Autoridad de Certificación Raíz de Confianza"**.

---
---

# 🇬🇧 ENGLISH README

## 🛡️ Rusty SASE Pro Gateway

## High-Performance Zero Trust SASE (Secure Access Service Edge) Platform

**Rusty SASE Pro Gateway** is a *minimal-core* implementation of an edge security gateway, designed to perform traffic filtering at native speed. Leveraging **Rust**'s asynchronous runtime (`Tokio`) and a decoupled control and data plane architecture, it offers professional-grade observability and ultra-low-latency policy management.

| **Status** | **Version** | **License** | **Author** |
| :---: | :---: | :---: | :---: |
| ✅ Stable | v1.0.2 | [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🚀 Key Features

| Feature | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Architecture** | Asynchronous Proxy (`tokio`, `hudsucker`) for high concurrency. | Reactive, Zero-Build (CDN-based Alpine.js + Tailwind CSS). |
| **NSS Observability** | **Structured JSON logs** (`tracing`) for SIEM/SOAR integration. | **SSE (Server-Sent Events)**: Live traffic stream (last 10 lines) and advanced filtering. |
| **Policies** | `tokio::sync::RwLock` | **Hot-Reload:** Blacklist changes are applied **instantly**. |
| **Security** | **Zero Trust Admin** (`127.0.0.1`) and TLS Inspection (CA generation with `rcgen`). | Professional UI with log filtering by IP/Domain/User-Agent. |

---

## 🔬 Detailed Technical Architecture

The Gateway's design focuses on **Service Availability (HA)** and **Policy Integrity**.

### 1. Data Plane (DP): Ultra-Low Latency Filtering Logic

The DP is powered by the asynchronous **Tokio** runtime and the `hudsucker` proxy framework.

| Mechanism | Technical Implementation | Cybersecurity Rationale |
| :--- | :--- | :--- |
| **Performance** | Rust Core (`tokio`) | Eliminates virtual machine *jitter* (Java/Go GC) to ensure predictable filtering latency, crucial for user experience and network responsiveness. |
| **L7 Inspection** | `rcgen` (CA/Certificate Generation) | Implements **SSL Inspection**. The Gateway intercepts the TLS handshake, dynamically signs certificates with its *in-memory* root CA, and allows domain filter application on encrypted traffic. |
| **Forensic Logs (NSS)**| `tracing-subscriber::json()` | Generates **structured JSON traffic logs** via `tracing`. Each event includes `src_ip`, `user_agent`, `domain`, and `action`, ensuring forensic-level traceability, ready for SIEM platform ingestion. |

### 2. Control Plane (CP): Atomic Configuration and Zero Trust

The CP securely manages configuration and provides the observability dashboard.

| Mechanism | Technical Implementation | Cybersecurity Rationale |
| :--- | :--- | :--- |
| **Policy Reload** | `tokio::sync::RwLock<AppState>` | **Atomic Hot-Reload**. The DP holds a constant read lock (`read().await`) for high concurrency. The admin interface acquires an **instantaneous** write lock (`write().await`) to apply policy changes, guaranteeing service consistency and high availability. |
| **Access Security** | `TcpListener::bind("127.0.0.1:0")` | Implements **Zero Trust Admin**. The management API is only accessible locally (localhost), isolating the management attack surface. The dynamic port avoids `AddrInUse` conflicts. |
| **Observability UX** | SSE (Server-Sent Events) | Real-time data *push* protocol for the *Traffic Feed* on the Dashboard. More efficient than polling for live log visualization. |

---

## 🛠️ Requirements and Installation

### Prerequisites

*   **Rust:** Version 1.70+
*   **Operating System:** Linux (required for network commands).
*   **Privileges:** `sudo` is required for execution.

### Deployment Steps

1.  **Clone the Repository (or structure files):**

    ```bash
    git clone https://github.com/yourusername/rusty-sase-pro-gateway
    cd rusty-sase-pro-gateway
    ```

2.  **Compile the optimized binary:**

    ```bash
    cargo build --release
    ```

3.  **Run the Gateway (Requires SUDO!):**

    ```bash
    # SUDO is required for port 8080 binding and log permissions.
    sudo ./target/release/rusty-sase-pro
    ```

> **NOTE:** The terminal will display the dynamic port. The browser will open automatically.

## 🌐 System Usage Guide

Upon execution, the browser will automatically open the Admin UI.

### 1. Policy and Dashboard Configuration

1.  **Dashboard:** Browser opens: `Admin UI: http://127.0.0.1:<Dynamic_Port>`.
2.  **Policy Management:** Add domains to the blacklist and save (instant Hot-Reload).
3.  **Analysis:** Use the dropdown menu and text field to filter the Traffic Feed by **Source IP, Domain, Method, or User-Agent**. Click a row to see full metadata details.

### 2. Client Proxy Configuration

To make filtering work, you must redirect client traffic to the Gateway:

*   **Proxy Host:** `<Your Machine's Local IP>` (e.g., `192.168.1.50`).
*   **Proxy Port:** `8080` (Fixed Data Plane Port).

### 3. Certificate Installation (HTTPS)

**Mandatory** for encrypted traffic:

1.  Download **`ca.crt`** from the Dashboard (🛡️ Certificado CA link).
2.  Install this file on your client device as a **"Trusted Root Certification Authority."**

---

---

# 🇨🇦 CATALAN README

## 🛡️ Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) d'Alt Rendiment

**Rusty SASE Pro Gateway** és una implementació *minimal-core* d'una passarel·la de seguretat de vora, dissenyada per executar el filtratge de trànsit a velocitat nativa. Utilitzant el *runtime* asíncron de **Rust** (`Tokio`) i una arquitectura de plans de control i dades desacoblats, ofereix observabilitat de nivell professional i gestió de polítiques de latència ultra baixa.

| **Estat** | **Versió** | **Llicència** | **Autor** |
| :---: | :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) | [![LinkedIn](https://img.shields.io/badge/LinkedIn-Ángel%20Urbina-0A66C2?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/angelurbina/) |

---

## 🚀 Característiques Clau

| Característica | Backend (Rust) | Frontend (Alpine/Tailwind) |
| :--- | :--- | :--- |
| **Arquitectura** | Proxy Asíncron (`tokio`, `hudsucker`) per a alta concurrència. | Reactiu, Zero-Build (Alpine.js + Tailwind CSS via CDN). |
| **Observabilitat NSS** | Registres **estructurats en JSON** (`tracing`) per a integració SIEM/SOAR. | **SSE (Server-Sent Events)**: Flux de trànsit en temps real (últimes 10 línies) i filtre avançat. |
| **Polítiques** | `tokio::sync::RwLock` | **Recàrrega en calent (Hot-Reload):** Els canvis a la llista negra s'apliquen **instantàniament**. |
| **Seguretat** | **Zero Trust Admin** (`127.0.0.1`) i Inspecció TLS (CA generation amb `rcgen`). | Interfície professional amb filtre de registres per IP/Domini/User-Agent. |

---

## 🔬 Arquitectura Tècnica Detallada

El disseny de la Passarel·la se centra en la **Disponibilitat del Servei (HA)** i la **Integritat de les Polítiques**.

### 1. Data Plane (DP): Lògica de Filtratge de Latència Ultra Baixa

El DP està impulsat pel *runtime* asíncron **Tokio** i el framework de proxy `hudsucker`.

| Mecanisme | Implementació Tècnica | Racional de Ciberseguretat |
| :--- | :--- | :--- |
| **Rendiment** | Rust Core (`tokio`) | Elimina el *jitter* de les màquines virtuals (Java/Go GC) per garantir una latència de filtratge predictible, crucial per a l'experiència de l'usuari i la resposta de xarxa. |
| **Inspecció L7** | `rcgen` (Generació de CA/Certificats) | Implementa **SSL Inspection**. La Passarel·la intercepta el *handshake* TLS, signa dinàmicament certificats amb la seva CA arrel generada *in-memory* i permet l'aplicació de filtres de domini al trànsit xifrat. |
| **Registres Forenses (NSS)**| `tracing-subscriber::json()` | Genera registres de trànsit **estructurats en format JSON** a través de `tracing`. Cada esdeveniment inclou `src_ip`, `user_agent`, `domain` i `action`, garantint una traçabilitat de nivell forense, llesta per a la ingesta en plataformes SIEM. |

### 2. Control Plane (CP): Configuració Atòmica i Zero Trust

El CP gestiona la configuració de forma segura i proporciona el panell d'observabilitat.

| Mecanisme | Implementació Tècnica | Racional de Ciberseguretat |
| :--- | :--- | :--- |
| **Recàrrega de Polítiques** | `tokio::sync::RwLock<AppState>` | **Hot-Reload Atòmic**. El DP manté un bloqueig de lectura constant (`read().await`) per a alta concurrència. La interfície d'administració pren un bloqueig d'escriptura (`write().await`) **instantani** per aplicar canvis de política, garantint la consistència i l'alta disponibilitat del servei. |
| **Seguretat d'Accés** | `TcpListener::bind("127.0.0.1:0")` | Implementa **Zero Trust Admin**. L'API de gestió només és accessible localment (localhost), aïllant la superfície d'atac de gestió. El port dinàmic evita conflictes de `AddrInUse`. |
| **Observabilitat UX** | SSE (Server-Sent Events) | Protocol de *push* de dades en temps real per al *Traffic Feed* al Panell. Més eficient que el *polling* per a la visualització de registres en viu. |

---

## 🛠️ Requisits i Instal·lació

### 1. Dependències (`Cargo.toml`)

Assegureu-vos que el vostre `Cargo.toml` conté aquestes dependències:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-stream = { version = "0.1", features = ["sync"] }
# ... (altres dependències)
opener = "0.7" 
```

### 2. Passos de Desplegament

1.  **Compilar el binari optimitzat:**
    ```bash
    cargo build --release
    ```
2.  **Executar la Passarel·la (Requereix SUDO!):**
    ```bash
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 Guia d'Ús del Sistema

En executar l'ordre, el navegador s'obrirà automàticament a l'Admin UI.

### 1. Configuració de Polítiques i Panell

1.  **Panell:** El navegador obrirà: `Admin UI: http://127.0.0.1:<Port_Dinàmic>`.
2.  **Filtrat de Registres:** Utilitzeu el menú desplegable i el camp de text per filtrar els esdeveniments per **IP Origen, Domini, Mètode o User-Agent**.
3.  **Detalls:** Feu clic a qualsevol fila de la taula per veure totes les metadades capturades.

### 2. Configuració del Proxy Client

Perquè el filtratge funcioni, heu de redirigir el trànsit dels clients al proxy:

*   **Host del Proxy:** `<La Vostra IP Local>`
*   **Port del Proxy:** `8080`
*   Instal·leu el certificat **`ca.crt`** (descarregable des del Panell) com a **"Autoritat de Certificació Arrel de Confiança"** als vostres dispositius per permetre el trànsit HTTPS.