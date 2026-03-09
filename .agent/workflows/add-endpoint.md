---
description: How to add a new API endpoint consistently
---

# Workflow: Add New API Endpoint

Follow these steps to ensure the backend remains "AI-Ready" and consistent.

## 1. Data Model
- If needed, update `src/models/entities/`.
- Use `derive(Serialize, Deserialize)`.
- Implement `is_valid()` if business validation is needed.

## 2. Contract (Trait)
- Add the necessary methods to the corresponding trait in `src/models/traits/`.

## 3. Persistence
- Implement the trait in the repository in `src/infrastructure/database/repositories/`.
- Remember to bind IDs using `.bind(("id", thing))`.

## 4. Service Layer
- Update the service in `src/application/services/` to include the new logic.

## 5. Interface (Handler)
- Create the handler in `src/interfaces/api/`.
- **Crucial:** Use `info!` logging at the start and end of the request.
- Return `HttpResponse` with standardized JSON for errors.

## 6. Route Registry
- Register the new route in `src/interfaces/api/routes.rs`.

## 7. Documentation (Introspection)
- Add the new endpoint (and payload examples) to `src/interfaces/api/api_doc.rs`.
- Verify by running `cargo run -- --list-api`.
