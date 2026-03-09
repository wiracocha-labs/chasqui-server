# Project Patterns & Technical Rules

This document serves as the "Source of Truth" for technical decisions in the Chasqui Server.

## 🆔 ID Management
- **Internal:** We use `surrealdb::sql::Thing`.
- **External:** All IDs exposed via API or JWT MUST be **pure UUIDs** (strings).
- **Transformation:** Use explicit extraction logic to remove the `⟨ ⟩` brackets from SurrealDB IDs.

## 🔐 Auth & JWT
- **Secret:** Must be loaded from `JWT_SECRET` env variable.
- **Subject (`sub`):** Contains the user's pure UUID.
- **Middleware:** Use `validate_token` for all protected routes.

## 📂 Architecture Layers
1. **Interfaces:** HTTP Handlers and Route config.
2. **Application:** Services that orchestrate business logic.
3. **Infrastructure:** Database repositories and WebSocket server.
4. **Models:** Entities and traits.

## 🛠️ Tooling
- **Validation:** Use `derive(Validate)` for all request DTOs.
- **Logging:** Use `log` macros (`info!`, `warn!`, `error!`).
- **Response Format:**
  - Success: JSON object.
  - Error: `{"type": "Error", "message": "..."}`
