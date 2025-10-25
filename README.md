# Chasqui Server

Backend in Rust (Actix-web) for secure data management and authentication. Optimized for performance, security, and development ease.

## 🚀 Features

- ✅ **RESTful API** with Actix-web 4.x
- 🔐 **JWT Authentication** with bcrypt
- 🗃️ **Database** SurrealDB
- 📝 **Structured logging** with different levels
- ⚙️ **Configuration** through environment variables
- 🛡️ **Data validation** with the `validator` crate
- 🔄 **Asynchronous operations** with async/await

## 🏗️ Project Structure

```
src/
├── application/    # Business logic
├── config/        # Application configuration
├── error/         # Error handling
├── infrastructure/# Technical implementations
│   └── database/  # SurrealDB connection and operations
├── interfaces/    # API controllers and routes
├── models/        # Data structures
└── lib.rs        # Library entry point
```

## 🚀 Getting Started

### Requirements

- Rust 1.70+
- SurrealDB (can run locally or in a container)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/chasqui-server.git
   cd chasqui-server
   ```

2. Configure environment variables (create a `.env` file):
   ```env
   # Server
   SERVER_HOST=127.0.0.1
   SERVER_PORT=8080
   
   # Database
   DATABASE_URL=ws://localhost:8000
   DATABASE_NS=chasqui
   DATABASE_DB=chasqui
   
   # Authentication
   JWT_SECRET=your_very_secure_secret_key
   JWT_EXPIRATION=86400  # seconds (24 hours)
   
   # Logging
   RUST_LOG=info
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## 🔒 Authentication

### Endpoints

- `POST /api/register` - User registration
- `POST /api/login` - User login

### JWT Flow

1. Client authenticates with email/username and password
2. Server responds with a signed JWT
3. Client includes the token in the `Authorization: Bearer <token>` header

## 🛡️ Security

### Authentication

✅ **Implemented**
- User registration and authentication
- Secure password hashing with bcrypt
- JWT tokens with expiration
- Basic input validation

📅 **Coming Soon**
- Refresh tokens
- Password recovery
- Two-factor authentication
- OAuth2/OpenID Connect

### 2. Authorization

✅ **Implemented**
- Basic roles in JWT
- Route protection with authentication

📅 **Coming Soon**
- Role-based access control (RBAC)
- Granular permissions

## 📊 Project Status

### Main Modules

| Module | Status | Description |
|--------|--------|-------------|
| REST API | ✅ Stable | Basic endpoints working |
| Authentication | ✅ Stable | JWT + bcrypt |
| Database | ✅ Stable | SurrealDB connection |
| Logging | ✅ Stable | Structured logging system |
| Validation | ✅ Stable | Input data validation |
| Webhooks | 🚧 In Development | In progress |
| Cache | 📅 Pending | To be implemented |

### 3. Data Protection
- Implemented
  - Bcrypt for passwords and no hash exposure in responses
  - User IDs as SurrealDB Thing user:<uuid-v4>
  - Configurable logging with APP_LOG_LEVEL
- Pending
  - Encryption of sensitive data at rest/in transit beyond TLS
  - Password complexity rules and more robust validations
  - Input sanitization/normalization at DTO level

### 4. Rate Limiting
- Implemented
  - —
- Pending
  - IP/user limits and brute force protection on login

### 5. Audit
- Implemented
  - Lifecycle logs (DB init, handlers, repos) and access logs (Actix Logger)
  - Verbosity control via APP_LOG_LEVEL (error|warn|info|debug|trace)
- Pending
  - Correlation with request IDs and structured logging (JSON)
  - Audit persistence and metrics (Prometheus)

### 6. API Security
- Implemented
  - Basic validation in DTOs (e.g., AddTaskRequest with validator)
- Pending
  - Explicit CORS, payload size limits, content types
  - Webhook validation/signatures
  - Stricter schema validations in endpoints

### 7. Infrastructure
- Implemented
  - —
- Pending
  - TLS/HTTPS (reverse proxy or native), HSTS and security headers
  - Timeouts and retry policies

### 8. Database
- Implemented
  - SurrealDB (WS) connection with parameterized queries
  - Users with Thing user:<uuid-v4> and password != NONE filter
- Pending
  - Unique indexes for username/email; migrations and seeds
  - Transactions/consistency where applicable

### 9. Testing

The project includes unit and integration tests to ensure code quality.

#### Test Structure

```
tests/
├── auth/               # Authentication tests
│   └── jwt_tests.rs    # JWT generation/validation tests
├── common/             # Shared test utilities
│   └── mod.rs         
├── config/             # Configuration tests
│   ├── config_tests.rs # Configuration loading tests
│   └── error_tests.rs  # Error handling tests
└── user/               # User model tests
    └── role_tests.rs   # Roles and permissions tests
```

#### Test Commands

Run all tests:
```bash
cargo test
```

Run specific test modules:
```bash
# Only authentication tests
cargo test auth::

# Only configuration tests
cargo test config::

# Only user model tests
cargo test user::
```

Useful options:
```bash
# Show test output (useful for logs)
cargo test -- --nocapture

# Run tests in a single thread (useful for debugging)
cargo test -- --test-threads=1

# Run a specific test by name
cargo test test_name
```

#### Test Configuration

Tests use an in-memory database to ensure isolation. The `tests/config/database_init_ignored.rs` file contains the database initialization configuration for tests.

#### Conventions

- Test files use the `_tests.rs` suffix
- Test modules follow the same structure as `src/`
- Tests should be independent and runnable in any order

#### Adding New Tests

1. Create a new file in the corresponding directory
2. Use `#[test]` for test functions
3. For async tests, use `#[actix_rt::test]`
4. Use `assert!`, `assert_eq!`, etc. for assertions

#### Debugging

To debug failed tests:
```bash
RUST_BACKTRACE=1 cargo test -- --nocapture
```

#### Test Coverage

To generate a coverage report (requires `cargo-tarpaulin`):
```bash
cargo tarpaulin --ignore-tests --out Html
```

## Debug Mode (Environment)
Control verbosity with a single environment variable:

Examples
- Development:
  - APP_LOG_LEVEL=trace
  - Optional: RUST_BACKTRACE=1
- Production:
  - APP_LOG_LEVEL=info

Notes
- APP_LOG_LEVEL unifies log configuration. You don't need RUST_LOG or APP_DEBUG.
- trace is the most verbose level (useful for development diagnostics).
- info shows operational information along with warn and error (recommended for production).

Note: APP_DEBUG is just an environment configuration flag. Adjust RUST_LOG according to your needs.

## 🤝 Contributing

Contributions are welcome. Please read our [contributing guidelines](CONTRIBUTING.md) before submitting a PR.

## 📄 License

This project is under the MIT license. See the [LICENSE](LICENSE) file for more details.