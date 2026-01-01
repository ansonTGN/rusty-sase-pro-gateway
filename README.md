# 🛡️ Rusty SASE Pro Gateway

<div align="center">

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg?logo=rust)
![Platform](https://img.shields.io/badge/platform-Linux-important)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Maintenance](https://img.shields.io/badge/maintained-yes-brightgreen.svg)

**Una arquitectura de seguridad de red Zero Trust de latencia ultrabaja.**
*Desacopla el Plano de Datos (Data Plane) del Plano de Control (Control Plane) para ofrecer seguridad a velocidad nativa.*

[🇪🇸 Español](#-descripción-general) • [🇬🇧 English](#-overview) • [Català](#-descripció-general)

</div>

---

## 🇪🇸 Descripción General

**Rusty SASE Pro Gateway** es una implementación de referencia de un *Secure Access Service Edge* (SASE) doméstico/profesional. A diferencia de los proxies tradicionales que sufren de latencia debido a la recolección de basura (GC) en lenguajes como Java o Go, este proyecto aprovecha el sistema de tipos y el modelo de memoria de **Rust** para garantizar un filtrado de paquetes determinista y sin pausas.

El sistema implementa una arquitectura de **Micro-Kernel de Seguridad**, donde la lógica de negocio (políticas) se inyecta atómicamente en el motor de red sin detener el flujo de tráfico, logrando una disponibilidad del 99.999% incluso durante actualizaciones de configuración.

### 👤 Autor y Mantenimiento

Este proyecto es diseñado y mantenido por **Ángel Urbina**.

[![LinkedIn](https://img.shields.io/badge/Connect_on-LinkedIn-0A66C2?style=for-the-badge&logo=linkedin)](https://www.linkedin.com/in/angelurbina/)

---

## 🔬 Arquitectura Técnica

El sistema sigue un diseño estricto de separación de responsabilidades:

```mermaid
%% Si GitHub no renderiza mermaid, el ASCII de abajo sirve de fallback
graph TD
    User[Usuario] -->|Tráfico HTTPS| DP[Data Plane :8080]
    DP -->|Filtrado| Internet
    DP -.->|Logs NSS| Disk[Logs JSON]
    DP -.->|Stream SSE| CP[Control Plane 127.0.0.1]
    Admin[Admin] -->|Políticas| CP
    CP -->|RwLock Write| DP