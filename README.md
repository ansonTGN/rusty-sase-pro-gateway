Este es el README profesional, detallado y multi-idioma solicitado.

---

# 🇪🇸 Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) de Alto Rendimiento

**Rusty SASE Pro Gateway** es una implementación *minimal-core* de un *gateway* de seguridad de borde, diseñado para ejecutar el filtrado de tráfico a velocidad nativa. Utilizando el runtime asíncrono de **Rust** (`Tokio`) y una arquitectura de planos de control y datos desacoplados, ofrece observabilidad de nivel profesional y gestión de políticas de latencia ultrabaja.

| **Estado** | **Versión** | **Licencia** | **Autor** |
| :---: | :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | MIT | Ángel Urbina |

---

## 🔬 Arquitectura Técnica Detallada (Nivel Dr. en Ciberseguridad)

El proyecto sigue el principio de separación de responsabilidades a través de dos planos desacoplados, priorizando el rendimiento del *data plane* sobre la latencia de gestión.

### 1. Data Plane (DP): Seguridad y Rendimiento (`0.0.0.0:8080`)

El DP está construido alrededor de un proxy *Man-in-the-Middle* con capacidad de inspección TLS de Capa 7 (L7).

| Componente | Mecanismo | Justificación en Ciberseguridad |
| :--- | :--- | :--- |
| **Proxy Core** | `hudsucker` (basado en `hyper`) | Framework de proxy de alto rendimiento que permite la intercepción de *handshakes* TLS (SSL Inspection) para aplicar políticas en el nombre de dominio (SNI) y la ruta completa. |
| **Lenguaje** | Rust (`tokio`) | Garantiza un entorno de ejecución **sin recolección de basura (GC)** ni *jitter*, lo que se traduce en una latencia predecible y extremadamente baja, crucial para el tráfico de red en tiempo real. |
| **Key Management** | `rcgen` | Generación *in-memory* de una Autoridad de Certificación (CA) de raíz para firmar dinámicamente certificados para los dominios interceptados. Esto permite la inspección L7 sin errores de certificado en el cliente. |
| **Logs (NSS)** | `tracing` | Genera logs de tráfico **estructurados en formato JSON** con campos críticos (`src_ip`, `domain`, `user_agent`, `action`). Esta salida es directamente integrable en cualquier sistema **SIEM/SOAR** (Splunk, Elastic) para análisis forense automatizado (Nanolog Streaming Service - NSS). |

### 2. Control Plane (CP): Gestión y Observabilidad (`127.0.0.1:<dinámico>`)

El CP proporciona la interfaz de gestión y los flujos de datos en tiempo real.

| Componente | Mecanismo | Justificación en Ciberseguridad |
| :--- | :--- | :--- |
| **Concurrencia** | `tokio::sync::RwLock<AppState>` | Mecanismo de **recarga en caliente atómica (Hot-Reload)**. El *data plane* toma un bloqueo de lectura (`read().await`) que permite que miles de hilos de conexión operen simultáneamente. La actualización de políticas (ej: añadir un dominio) toma un bloqueo de escritura (`write().await`) **instantáneamente**, asegurando que los cambios son efectivos sin interrumpir ninguna conexión activa. |
| **Seguridad de Acceso** | `TcpListener::bind("127.0.0.1:0")` | Implementación del principio de **Zero Trust Admin**. El *Control Plane* sólo está disponible localmente (localhost), impidiendo el acceso remoto a la gestión de políticas. El puerto se asigna de forma **efímera y dinámica** para evitar errores de `AddrInUse`. |
| **Dashboard** | SSE (Server-Sent Events) | Protocolo de eventos *push* en tiempo real para el *Traffic Feed*. Más ligero que WebSockets y optimizado para la baja latencia de los datos de observabilidad. |

---

## 🛠️ Requisitos y Despliegue

### Requisitos Previos

*   **Rust:** Versión 1.70+
*   **Sistema Operativo:** Linux
*   **Privilegios:** `sudo` es obligatorio para ejecutar el binario.

### Instalación y Ejecución

1.  **Compilar el binario optimizado:**
    ```bash
    cargo build --release
    ```
2.  **Ejecutar la Passarel·la (¡Requiere SUDO!):**
    ```bash
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 Guía de Uso del Sistema

Al ejecutar el comando, el navegador se abrirá automáticamente en el Admin UI.

### 1. Configuración de Políticas y Dashboard
*   El navegador abrirá: `Admin UI: http://127.0.0.1:<Puerto_Dinámico>`.
*   En el Dashboard, gestione la lista negra y guarde. Los cambios son instantáneos.

### 2. Configuración del Cliente (Proxy)
Para que el Gateway funcione, configure sus dispositivos para usar el puerto `8080`.
*   **Host del Proxy:** `<IP Local de su máquina>`
*   **Puerto del Proxy:** `8080`

### 3. Instalación del Certificado (HTTPS)
*   En el Dashboard, haga clic en **🛡️ Certificado**.
*   Instale el archivo `ca.crt` en su dispositivo como **"Autoridad de Certificación Raíz de Confianza"** para evitar errores de conexión cifrada.

---

## 👤 Atribución del Proyecto

Este proyecto es una implementación de código abierto mantenida y diseñada por:

| **Autor** | **Perfil Profesional** |
| :--- | :--- |
| Ángel Urbina | [https://www.linkedin.com/in/angelurbina/](https://www.linkedin.com/in/angelurbina/) |

---
---

# 🇬🇧 ENGLISH README

## 🛡️ Rusty SASE Pro Gateway

## High-Performance Zero Trust SASE (Secure Access Service Edge) Platform

**Rusty SASE Pro Gateway** is a *minimal-core* implementation of an edge security gateway, designed to perform traffic filtering at native speed. Leveraging **Rust**'s asynchronous runtime (`Tokio`) and a decoupled control and data plane architecture, it offers professional-grade observability and ultra-low-latency policy management.

| **Status** | **Version** | **License** | **Author** |
| :---: | :---: | :---: | :---: |
| ✅ Stable | v1.0.2 | MIT | Ángel Urbina |

---

## 🔬 Detailed Technical Architecture (Cybersecurity Ph.D. Level)

The project adheres to the principle of separation of concerns through two decoupled planes, prioritizing data plane performance over management latency.

### 1. Data Plane (DP): Security and Performance (`0.0.0.0:8080`)

The DP is built around a Man-in-the-Middle proxy with Layer 7 (L7) TLS inspection capability.

| Component | Mechanism | Cybersecurity Rationale |
| :--- | :--- | :--- |
| **Proxy Core** | `hudsucker` (based on `hyper`) | High-performance proxy framework enabling TLS handshake interception (SSL Inspection) to apply policies based on the full domain name (SNI) and path. |
| **Language** | Rust (`tokio`) | Guarantees a **garbage collection (GC)-free** execution environment with no *jitter*, resulting in predictable and extremely low latency—critical for real-time network traffic. |
| **Key Management** | `rcgen` | *In-memory* generation of a root Certificate Authority (CA) to dynamically sign certificates for intercepted domains, enabling L7 inspection without client certificate errors. |
| **Logs (NSS)** | `tracing` | Generates **structured JSON traffic logs** with critical fields (`src_ip`, `domain`, `user_agent`, `action`). This output is directly integrable into any **SIEM/SOAR** system (Splunk, Elastic) for automated forensic analysis (Nanolog Streaming Service - NSS). |

### 2. Control Plane (CP): Management and Observability (`127.0.0.1:<dynamic>`)

The CP provides the management interface and real-time data flows.

| Component | Mechanism | Cybersecurity Rationale |
| :--- | :--- | :--- |
| **Concurrency** | `tokio::sync::RwLock<AppState>` | **Atomic Hot-Reload** mechanism. The data plane holds a read lock (`read().await`), allowing thousands of connection threads to operate simultaneously. Policy updates (e.g., adding a domain) acquire a **write lock instantaneously**, ensuring changes are effective without dropping any active connections. |
| **Access Security** | `TcpListener::bind("127.0.0.1:0")` | Implements the **Zero Trust Admin** principle. The Control Plane is only accessible locally (localhost), preventing remote access to policy management even if the machine is network-exposed. The port is assigned **ephemerally and dynamically** to avoid `AddrInUse` errors. |
| **Dashboard** | SSE (Server-Sent Events) | Real-time *push* event protocol for the *Traffic Feed*. Lighter than WebSockets and optimized for low-latency observability data. |

---

## 🛠️ Requirements and Deployment

### Prerequisites

*   **Rust:** Version 1.70+
*   **Operating System:** Linux
*   **Privileges:** `sudo` is mandatory to run the binary.

### Installation and Execution

1.  **Compile the optimized binary:**
    ```bash
    cargo build --release
    ```
2.  **Run the Gateway (Requires SUDO!):**
    ```bash
    sudo ./target/release/rusty-sase-pro
    ```

## 🌐 System Usage Guide

Upon execution, the browser will automatically open the Admin UI, and the proxy will become active.

### 1. Policy and Dashboard Configuration
*   The browser opens: `Admin UI: http://127.0.0.1:<Dynamic_Port>`.
*   In the Dashboard, manage the blacklist and save. Changes are instantaneous.

### 2. Client Configuration (Proxy)
To filter traffic, configure clients to use port `8080`.
*   **Proxy Host:** `<Your Machine's Local IP>`
*   **Proxy Port:** `8080`

### 3. Certificate Installation (HTTPS)
*   In the Dashboard, click **🛡️ Certificado**.
*   Install the `ca.crt` file on your client device as a **"Trusted Root Certification Authority"** to avoid encrypted connection errors.

---

## 👤 Project Attribution

This open-source implementation is maintained and designed by:

| **Author** | **Professional Profile** |
| :--- | :--- |
| Ángel Urbina | [https://www.linkedin.com/in/angelurbina/](https://www.linkedin.com/in/angelurbina/) |

---
---

# CATALAN README

## 🛡️ Rusty SASE Pro Gateway

## Plataforma Zero Trust SASE (Secure Access Service Edge) d'Alt Rendiment

**Rusty SASE Pro Gateway** és una implementació *minimal-core* d'una passarel·la de seguretat de vora, dissenyada per executar el filtratge de trànsit a velocitat nativa. Utilitzant el *runtime* asíncron de **Rust** (`Tokio`) i una arquitectura de plans de control i dades desacoblats, ofereix observabilitat de nivell professional i gestió de polítiques de latència ultra baixa.

| **Estat** | **Versió** | **Llicència** | **Autor** |
| :---: | :---: | :---: | :---: |
| ✅ Estable | v1.0.2 | MIT | Ángel Urbina |

---

## 🔬 Arquitectura Tècnica Detallada (Nivell Dr. en Ciberseguretat)

El projecte segueix el principi de separació de responsabilitats a través de dos plans desacoblats, prioritzant el rendiment del *data plane* sobre la latència de gestió.

### 1. Data Plane (DP): Seguretat i Rendiment (`0.0.0.0:8080`)

El DP està construït al voltant d'un proxy *Man-in-the-Middle* amb capacitat d'inspecció TLS de Capa 7 (L7).

| Component | Mecanisme | Justificació en Ciberseguretat |
| :--- | :--- | :--- |
| **Proxy Core** | `hudsucker` (basat en `hyper`) | Framework de proxy d'alt rendiment que permet la intercepció de *handshakes* TLS (SSL Inspection) per aplicar polítiques basades en el nom de domini (SNI) i la ruta completa. |
| **Llenguatge** | Rust (`tokio`) | Garanteix un entorn d'execució **sense recol·lecció d'escombraries (GC)** ni *jitter*, cosa que es tradueix en una latència predictible i extremadament baixa, crucial per al trànsit de xarxa en temps real. |
| **Key Management** | `rcgen` | Generació *in-memory* d'una Autoritat de Certificació (CA) d'arrel per signar dinàmicament certificats per als dominis interceptats. Això permet la inspecció L7 sense errors de certificat al client. |
| **Logs (NSS)** | `tracing` | Genera registres de trànsit **estructurats en format JSON** amb camps crítics (`src_ip`, `domain`, `user_agent`, `action`). Aquesta sortida és directament integrable a qualsevol sistema **SIEM/SOAR** (Splunk, Elastic) per a anàlisi forense automatitzada (Nanolog Streaming Service - NSS). |

### 2. Control Plane (CP): Gestió i Observabilitat (`127.0.0.1:<dinàmic>`)

El CP proporciona la interfície de gestió i els fluxos de dades en temps real.

| Component | Mecanisme | Justificació en Ciberseguretat |
| :--- | :--- | :--- |
| **Concurrència** | `tokio::sync::RwLock<AppState>` | Mecanisme de **recàrrega en calent atòmica (Hot-Reload)**. El *data plane* pren un bloqueig de lectura (`read().await`) que permet que milers de fils de connexió operin simultàniament. L'actualització de polítiques (ex: afegir un domini) pren un bloqueig d'escriptura (`write().await`) **instantàniament**, assegurant que els canvis són efectius sense interrompre cap connexió activa. |
| **Seguretat d'Accés** | `TcpListener::bind("127.0.0.1:0")` | Implementa el principi de **Zero Trust Admin**. El *Control Plane* només és accessible localment (localhost), impedint l'accés remot a la gestió de polítiques. El port s'assigna de forma **efímera i dinàmica** per evitar errors de `AddrInUse`. |
| **Dashboard** | SSE (Server-Sent Events) | Protocol d'esdeveniments *push* en temps real per al *Traffic Feed*. Més lleuger que WebSockets i optimitzat per a la baixa latència de les dades d'observabilitat. |

---

## 🛠️ Requisits i Desplegament

### Requisits Previs

*   **Rust:** Versió 1.70+
*   **Sistema Operatiu:** Linux
*   **Privilegis:** `sudo` és obligatori per executar el binari.

### Instal·lació i Execució

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
*   El navegador obrirà: `Admin UI: http://127.0.0.1:<Port_Dinàmic>`.
*   Al Panell, gestioneu la llista negra i deseu. Els canvis són instantanis.

### 2. Configuració del Client (Proxy)
Per filtrar el trànsit, configureu els clients per utilitzar el port `8080`.
*   **Host del Proxy:** `<La Vostra IP Local>`
*   **Port del Proxy:** `8080`

### 3. Instal·lació del Certificat (HTTPS)
*   Al Panell, feu clic a **🛡️ Certificado**.
*   Instal·leu el fitxer `ca.crt` al vostre dispositiu com a **"Autoritat de Certificació Arrel de Confiança"** per evitar errors de connexió xifrada.

---

## 👤 Atribució del Projecte

Aquesta implementació de codi obert és mantinguda i dissenyada per:

| **Autor** | **Perfil Professional** |
| :--- | :--- |
| Ángel Urbina | [https://www.linkedin.com/in/angelurbina/](https://www.linkedin.com/in/angelurbina/) |