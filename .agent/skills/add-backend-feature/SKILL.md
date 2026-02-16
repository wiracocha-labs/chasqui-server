# Skill: Add Backend Feature (Chasqui Monolith)

This skill guides you through the process of adding a new feature (endpoint + logic) to the Chasqui Server monolith safely and consistently.

## Prerequisites
- Knowledge of the [Architecture](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/knowledge/architecture.md)
- Adherence to [Coding Patterns](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/knowledge/patterns.md)

## Implementation Steps

### 1. Model Definition (`src/models/entities/`)
Define the entity and its DTOs. 
- Use `serde` for serialization.
- Ensure UUIDs are handled as clean strings in DTOs.

### 2. Repository Layer (`src/infrastructure/database/repositories/`)
- Create a new repository trait and its SurrealDB implementation.
- Filter out legacy/incomplete rows in queries.

### 3. Service Layer (`src/application/services/`)
- Implement the business logic.
- Services must be initialized in `main.rs` and wrapped in `Arc` + `web::Data`.

### 4. Interface Layer (`src/interfaces/api/`)
- Create a new handler file.
- Register the routes in `routes.rs`.

### 5. Documentation & Verification
- Update the API documentation handler.
- Run `cargo run -- --list-api` to verify the new endpoint is listed.

## Code Example: Service Injection
```rust
// In main.rs
let my_service = Arc::new(MyService::new(repo.clone()));
let my_service_data = web::Data::from(my_service.clone());

App::new()
    .app_data(my_service_data.clone())
```

## Hybrid Evolution Hint
If this feature is expected to consume high CPU (like Image Processing or AI), consider designing the Service layer to communicate with an external microservice via HTTP/gRPC instead of doing the work locally.
