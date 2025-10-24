# Chasqui Server

Backend en Rust (Actix-web) para manejo seguro de datos y autenticación. Optimizado para rendimiento, seguridad y facilidad de desarrollo.

## 🚀 Características

- ✅ **API RESTful** con Actix-web 4.x
- 🔐 **Autenticación JWT** con bcrypt
- 🗃️ **Base de datos** SurrealDB
- 📝 **Logging estructurado** con diferentes niveles
- ⚙️ **Configuración** mediante variables de entorno
- 🛡️ **Validación de datos** con la crate `validator`
- 🔄 **Operaciones asíncronas** con async/await

## 🏗️ Estructura del Proyecto

```
src/
├── application/    # Lógica de negocio
├── config/        # Configuración de la aplicación
├── error/         # Manejo de errores
├── infrastructure/# Implementaciones técnicas
│   └── database/  # Conexión y operaciones con SurrealDB
├── interfaces/    # Controladores y rutas de la API
├── models/        # Estructuras de datos
└── lib.rs        # Punto de entrada de la biblioteca
```

## 🚀 Comenzando

### Requisitos

- Rust 1.70+
- SurrealDB (puede ejecutarse localmente o en contenedor)

### Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/chasqui-server.git
   cd chasqui-server
   ```

2. Configura las variables de entorno (crea un archivo `.env`):
   ```env
   # Servidor
   SERVER_HOST=127.0.0.1
   SERVER_PORT=8080
   
   # Base de datos
   DATABASE_URL=ws://localhost:8000
   DATABASE_NS=chasqui
   DATABASE_DB=chasqui
   
   # Autenticación
   JWT_SECRET=tu_clave_secreta_muy_segura
   JWT_EXPIRATION=86400  # segundos (24 horas)
   
   # Logging
   RUST_LOG=info
   ```

3. Ejecuta el servidor:
   ```bash
   cargo run
   ```

## 🔒 Autenticación

### Endpoints

- `POST /api/register` - Registro de usuario
- `POST /api/login` - Inicio de sesión

### Flujo JWT

1. El cliente se autentica con email/username y contraseña
2. El servidor responde con un JWT firmado
3. El cliente incluye el token en el header `Authorization: Bearer <token>`

## 🛡️ Seguridad

### Autenticación

✅ **Implementado**
- Registro y autenticación de usuarios
- Hash seguro de contraseñas con bcrypt
- Tokens JWT con expiración
- Validación básica de entrada

📅 **Próximamente**
- Refresh tokens
- Recuperación de contraseña
- Autenticación de dos factores
- OAuth2/OpenID Connect

### 2. Autorización

✅ **Implementado**
- Roles básicos en JWT
- Protección de rutas con autenticación

📅 **Próximamente**
- Control de acceso basado en roles (RBAC)
- Permisos granulares

## 📊 Estado del Proyecto

### Módulos Principales

| Módulo | Estado | Descripción |
|--------|--------|-------------|
| API REST | ✅ Estable | Endpoints básicos funcionando |
| Autenticación | ✅ Estable | JWT + bcrypt |
| Base de Datos | ✅ Estable | Conexión con SurrealDB |
| Logging | ✅ Estable | Sistema de logs estructurado |
| Validación | ✅ Estable | Validación de datos de entrada |
| Webhooks | 🚧 En desarrollo | En implementación |
| Caché | 📅 Pendiente | Por implementar |

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

El proyecto incluye pruebas unitarias y de integración para garantizar la calidad del código.

#### Estructura de Pruebas

```
tests/
├── auth/               # Pruebas de autenticación
│   └── jwt_tests.rs    # Pruebas de generación/validación de JWT
├── common/             # Utilidades compartidas para pruebas
│   └── mod.rs         
├── config/             # Pruebas de configuración
│   ├── config_tests.rs # Pruebas de carga de configuración
│   └── error_tests.rs  # Pruebas de manejo de errores
└── user/               # Pruebas de modelos de usuario
    └── role_tests.rs   # Pruebas de roles y permisos
```

#### Comandos de Prueba

Ejecutar todas las pruebas:
```bash
cargo test
```

Ejecutar pruebas específicas por módulo:
```bash
# Solo pruebas de autenticación
cargo test auth::

# Solo pruebas de configuración
cargo test config::

# Solo pruebas de modelos de usuario
cargo test user::
```

Opciones útiles:
```bash
# Mostrar salida de las pruebas (útil para ver logs)
cargo test -- --nocapture

# Ejecutar pruebas en un solo hilo (útil para debugging)
cargo test -- --test-threads=1

# Ejecutar una prueba específica por nombre
cargo test nombre_de_la_prueba
```

#### Configuración para Pruebas

Las pruebas utilizan una base de datos en memoria para garantizar aislamiento. El archivo `tests/config/database_init_ignored.rs` contiene la configuración de inicialización de la base de datos para pruebas.

#### Convenciones

- Los archivos de prueba usan el sufijo `_tests.rs`
- Los módulos de prueba siguen la misma estructura que `src/`
- Las pruebas deben ser independientes y poder ejecutarse en cualquier orden

#### Agregando Nuevas Pruebas

1. Crea un nuevo archivo en el directorio correspondiente
2. Usa `#[test]` para funciones de prueba
3. Para pruebas asíncronas, usa `#[actix_rt::test]`
4. Usa `assert!`, `assert_eq!`, etc. para las aserciones

#### Depuración

Para depurar pruebas fallidas:
```bash
RUST_BACKTRACE=1 cargo test -- --nocapture
```

#### Cobertura de Pruebas

Para generar un informe de cobertura (requiere `cargo-tarpaulin`):
```bash
cargo tarpaulin --ignore-tests --out Html
```


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


## 🤝 Contributing

Contributions are welcome. Please read our [contributing guidelines](CONTRIBUTING.md) before submitting a PR.

## 📄 License

This project is under the MIT license. See the [LICENSE](LICENSE) file for more details.