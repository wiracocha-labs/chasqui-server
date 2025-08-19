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
- Sistema de login/registro
- Manejo de sesiones
- JWT (JSON Web Tokens):
  - Algoritmo: HS256/RS256
  - Rotación de claves
  - Refresh tokens
  - Token blacklisting
  - Tiempo de expiración configurable
  - Claims mínimos necesarios

### 2. Autorización
- Control de acceso basado en roles (RBAC)
- Permisos granulares
- Validación de rutas protegidas

### 3. Protección de Datos
- Encriptación de datos sensibles
- Hash seguro de contraseñas:
  - Algoritmo: bcrypt
  - Salt único por usuario
  - Parámetros de memoria/tiempo configurables
  - Validación de fortaleza de contraseña
- Sanitización de entradas

### 4. Rate Limiting
- Límites por IP
- Límites por usuario
- Ventanas de tiempo configurables

### 5. Auditoría
- Logging de eventos de seguridad
- Registro de intentos de acceso
- Trazabilidad de acciones

### 6. Seguridad de APIs
- Validación de webhooks
- CORS configurado
- Límites de tamaño en payloads

### 7. Infraestructura
- TLS/HTTPS
- Headers de seguridad
- Timeouts configurables
