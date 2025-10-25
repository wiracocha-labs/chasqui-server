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

# 🖼️ Image Management Strategy

## 📋 Architecture: Direct URL

Images are stored as files on disk and the database only stores URLs (strings). The frontend accesses images directly without going through the backend.

```
Backend → Saves image → Returns URL string
Frontend → Uses URL directly in <img src="url">
```

### Advantages of this architecture:
- ✅ **Performance**: Web server (nginx/actix-files) serves static files very fast
- ✅ **Automatic caching**: Browser and CDN cache without additional configuration
- ✅ **Scalability**: Doesn't overload backend with byte transfer
- ✅ **Easy migration**: Changing storage only requires updating URLs in DB

## 🚀 Implementation Roadmap

### 🟢 PHASE 1: MVP / Development (0-6 months)
**Stack**: Local Files

```toml
[dependencies]
actix-files = "0.6"        # Serve static files
actix-multipart = "0.6"    # Receive uploads
tokio = { version = "1", features = ["fs", "io-util"] }
mime = "0.3"               # MIME types
```

**Implementation**:
- `./uploads/` folder on server
- `actix-files` serves static content
- DB stores URLs as strings
- Frontend accesses directly

**Characteristics**:
- 💰 **Cost**: $0
- 🔧 **Complexity**: Low
- 📊 **Recommended limits**:
  - Up to 50GB of images
  - Up to 10,000 users
  - Traffic < 1TB/month

**DB Structure**:
```rust
#[derive(Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    image_url: String,  // "http://localhost:8080/uploads/abc.jpg"
}
```

### 🟡 PHASE 2: Growth (6-18 months)
**Stack**: Local Files + Cloudflare CDN

**What to do**:
1. Point domain to Cloudflare (free)
2. Enable orange cloud proxy for `/uploads/*`
3. No backend code changes
4. Cloudflare caches images globally

**Characteristics**:
- 💰 **Cost**: $0
- 🔧 **Complexity**: Low
- 🌍 **Free global CDN**
- 🔒 **Automatic HTTPS**
- 🛡️ **DDoS protection**
- 📊 **Recommended limits**:
  - Up to 200GB of images
  - Up to 100,000 users
  - Traffic < 5TB/month

### 🔴 PHASE 3: Scale / High Production (18+ months)
**Stack**: Cloudflare R2

**When to migrate**:
- Backend on multiple servers
- > 200GB of images
- Traffic > 5TB/month
- Need automatic backups
- Critical high availability

**Additional dependencies**:
```toml
[dependencies]
aws-sdk-s3 = "1.52"    # R2 is S3-compatible
aws-config = "1.5"
```

**Characteristics**:
- 💰 **Cost**: ~$15/month for 1TB
- 🔧 **Complexity**: Medium
- 🌐 **FREE bandwidth (unlimited)**
- 📈 **99.9% uptime SLA**
- 🌍 **Integrated Cloudflare CDN**
- 💾 **Automatic backups**
- 🌎 **Multi-region**

**Migration process**:
1. Script to upload local files to R2
2. Update URLs in DB (string → string)
3. Frontend no changes
4. Keep local files as temporary backup

## 💰 Cost Comparison

| Service | Storage | Bandwidth | Total/month |
|---------|---------|-----------|-------------|
| **Local Files** | $0 | $0* | $0* |
| **Cloudflare R2** | $1.50 | $0 | $1.50 |
| **Backblaze B2** | $0.60 | $10 | $10.60 |
| **AWS S3** | $2.30 | $90 | $92.30 |
| **Cloudinary** | - | - | $89+ |

*Included in server, limited to server disk capacity

## 🎯 Key Principles

### 1. Always URLs as Strings
```rust
// In DB ALWAYS:
struct Product {
    image_url: String  // Whether local or cloud
}

// Phase 1: "http://localhost:8080/uploads/abc.jpg"
// Phase 2: "https://yourdomain.com/uploads/abc.jpg"
// Phase 3: "https://yourdomain.r2.dev/images/abc.jpg"
```

### 2. Frontend NEVER changes
```jsx
// This works in ALL phases
<img src={product.image_url} />
```

### 3. Migration is backend only
1. Upload files to new destination
2. UPDATE URLs in DB
3. Frontend untouched

### 4. No vendor lock-in
- R2 is S3-compatible
- Can easily switch to AWS/Backblaze
- Local files always as fallback

## 📊 Decision Tree

```
Do you have < 50GB of images?
└── Yes → Local files
    └── Do you have > 1,000 users/day?
        └── Yes → + Free Cloudflare CDN
        └── No → Stay as is

Do you have > 200GB of images?
└── Migrate to Cloudflare R2
```

## 📝 Implementation Example (Phase 1)

### Backend - Upload endpoint:
```rust
use actix_multipart::Multipart;
use futures::StreamExt;
use uuid::Uuid;

async fn upload_image(
    mut payload: Multipart,
    db: web::Data<Surreal<Any>>
) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        
        let uuid = Uuid::new_v4();
        let filename = format!("{}.jpg", uuid);
        let filepath = format!("./uploads/{}", filename);
        
        // Save file
        let mut f = tokio::fs::File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            tokio::io::AsyncWriteExt::write_all(&mut f, &data).await?;
        }
        
        // Public URL
        let image_url = format!("http://localhost:8080/uploads/{}", filename);
        
        return Ok(HttpResponse::Ok().json(json!({
            "image_url": image_url  // ← STRING for frontend
        })));
    }
    
    Ok(HttpResponse::BadRequest().finish())
}
```

### Backend - Serve static files:
```rust
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // API routes
            .route("/api/upload", web::post().to(upload_image))
            
            // Serve static files
            .service(
                fs::Files::new("/uploads", "./uploads")
                    .show_files_listing()
                    .use_last_modified(true)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Frontend - React example:
```jsx
// Upload
async function uploadImage(file) {
  const formData = new FormData();
  formData.append('image', file);
  
  const res = await fetch('/api/upload', {
    method: 'POST',
    body: formData
  });
  
  const { image_url } = await res.json();
  return image_url;  // ← STRING
}

// Display image
function ProductCard({ product }) {
  return (
    <img src={product.image_url} alt={product.name} />
    //       ↑ Direct URL, no proxy
  );
}
```

## 🔒 Special Use Cases

### Private images with authentication:
```rust
async fn get_private_image(
    path: web::Path<String>,
    auth: BearerAuth
) -> Result<HttpResponse, Error> {
    verify_token(&auth)?;
    let bytes = tokio::fs::read(format!("./private/{}", path)).await?;
    Ok(HttpResponse::Ok()
        .content_type(mime::IMAGE_JPEG)
        .body(bytes))
}
```

### On-the-fly transformation:
```rust
// Dynamic resize: /images/abc.jpg?size=thumbnail
async fn get_image_resized(
    path: web::Path<String>,
    query: web::Query<ImageQuery>
) -> Result<HttpResponse, Error> {
    let image = image::open(path)?;
    let resized = image.resize(...);
    Ok(HttpResponse::Ok().body(resized))
}
```

## ✅ Summary
- **Phase 1-2**: Perfect for development, MVP and first 1-2 years
- **Current stack**: Supports up to 100,000 users
- **Migration**: Only add aws-sdk-s3 when you scale
- **Frontend**: Never changes, just uses URLs
- **Initial cost**: $0

## 🤝 Contributing

Contributions are welcome. Please read our [contributing guidelines](CONTRIBUTING.md) before submitting a PR.

## 📄 License

This project is under the MIT license. See the [LICENSE](LICENSE) file for more details.