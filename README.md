# chasqui-server
Chasqui Server es un backend en Rust (Actix-web) para manejar webhooks, integraciones y procesamiento seguro de datos. Optimizado para bajo consumo y alta velocidad, potencia la experiencia descentralizada de Chasqui con foco en privacidad y colaboración abierta.

## Características

- Sistema robusto de webhooks con validación y rate limiting
- Cache optimizado para alto rendimiento
- Sistema de logging estructurado
- Seguridad mejorada con validación de firmas
- Manejo eficiente de recursos

## Módulos

- `webhooks`: Gestión de webhooks y callbacks
- `cache`: Sistema de caché en memoria y disco
- `logging`: Logging estructurado
- `security`: Validación y seguridad
- `models`: Modelos de dominio
- `interfaces`: APIs y endpoints
- `infrastructure`: Implementaciones técnicas

## Seguridad

### 1. Autenticación
- Implementado
  - Registro y login:
    - POST /api/register (valida username alfabético y email básico)
    - POST /api/login (acepta email o username + password; fallback entre ambos)
  - Hash de contraseñas con bcrypt (BCRYPT_COST configurable)
  - JWT HS256 con SECRET_KEY (obligatorio) y expiración via JWT_EXP_SECONDS
  - Claims: sub (uuid), iat, exp, username, roles (por defecto ["user"])
  - Compatibilidad legacy: User.password y User.email opcionales; consultas filtran filas sin hash con password != NONE
- Pendiente
  - Refresh tokens, rotación de claves, blacklist/invalidación de tokens, logout
  - Recuperación/cambio de contraseña
  - Validación estricta de email (validator) y unicidad de username/email

### 2. Autorización
- Implementado
  - roles incluido en el JWT (valor por defecto)
- Pendiente
  - Middleware/guards RBAC por ruta; verificación de roles/permisos granulares

### 3. Protección de Datos
- Implementado
  - Bcrypt para contraseñas y no exposición del hash en respuestas
  - IDs de usuarios como SurrealDB Thing user:<uuid-v4>
  - Logging configurable con APP_LOG_LEVEL
- Pendiente
  - Cifrado de datos sensibles en reposo/traslado adicional al TLS
  - Reglas de complejidad de contraseñas y validaciones más robustas
  - Sanitización/normalización de entradas a nivel de DTO

### 4. Rate Limiting
- Implementado
  - —
- Pendiente
  - Límites por IP/usuario y protección de fuerza bruta en login

### 5. Auditoría
- Implementado
  - Logs del ciclo de vida (DB init, handlers, repos) y access logs (Actix Logger)
  - Control de verbosidad por APP_LOG_LEVEL (error|warn|info|debug|trace)
- Pendiente
  - Correlación con request IDs y logging estructurado (JSON)
  - Persistencia de auditoría y métricas (Prometheus)

### 6. Seguridad de APIs
- Implementado
  - Validación básica en DTOs (p. ej., AddTaskRequest con validator)
- Pendiente
  - CORS explícito, límites de tamaño de payload, tipos de contenido
  - Validación/firmas de webhooks
  - Validaciones de esquema más estrictas en endpoints

### 7. Infraestructura
- Implementado
  - —
- Pendiente
  - TLS/HTTPS (reverse proxy o nativo), HSTS y security headers
  - Timeouts y políticas de reintentos

### 8. Base de Datos
- Implementado
  - Conexión SurrealDB (WS) con consultas parametrizadas
  - Usuarios con Thing user:<uuid-v4> y filtro password != NONE
- Pendiente
  - Índices únicos para username/email; migraciones y seeds
  - Transacciones/consistencia donde aplique

### 9. Pruebas
- Implementado
  - Tests de errores (mapeo HTTP), JWT y configuración; doctests ajustados
- Pendiente
  - Tests de flujo de autenticación/autorización y e2e de API

## Modo Debug (entorno)
Controla la verbosidad con una sola variable de entorno:

Ejemplos
- Desarrollo:
  - APP_LOG_LEVEL=trace
  - Opcional: RUST_BACKTRACE=1
- Producción:
  - APP_LOG_LEVEL=info

Notas
- APP_LOG_LEVEL unifica la configuración de logs. No necesitas RUST_LOG ni APP_DEBUG.
- trace es el nivel más verboso (útil para diagnóstico en desarrollo).
- info muestra información operativa junto a warn y error (recomendado en producción).

Nota: APP_DEBUG es solo una bandera de configuración del entorno. Ajusta RUST_LOG según tus necesidades.
