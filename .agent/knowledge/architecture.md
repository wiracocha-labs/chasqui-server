# System Architecture (AI-Ready)

This document explains how the code is organized. AI Agents must strictly follow this structure.

## 🏛️ Dependency Flow
`Interfaces` -> `Application` -> `Infrastructure` -> `Models`

### 1. Interfaces (Entry Points)
- Located in `src/interfaces/api/`.
- **Role:** Handle HTTP/WS requests, validate inputs with `validator`, and return JSON.
- **Constraints:** NO business logic here. Call services.

### 2. Application (Business Logic)
- Located in `src/application/services/`.
- **Role:** Orchestrate operations. Use repositories to fetch data and apply rules.
- **Constraints:** Tech-agnostic. Should not know about Actix or SQL directly (use traits).

### 3. Infrastructure (Side Effects)
- Located in `src/infrastructure/`.
- **Role:** Implement trait definitions for Database, WS Hub, Auth, etc.
- **Repositories:** `src/infrastructure/database/repositories/`.

### 4. Models (Data)
- Located in `src/models/`.
- **Role:** Plain structs and trait definitions.

## Hybrid Architecture: Core vs. Satellites

To maintain high performance and low costs, we distinguish between the **Monolith Core** and **Satellite Microservices**.

### 🏛️ The Monolith Core (Chasqui Server)
Handled by this Rust repository. It is responsible for high-performance, real-time, and state-critical operations:
- **Identity & Auth**: User registration, login, and JWT management.
- **Messaging Engine**: WebSocket hub, message persistence, and conversation state.
- **Task Management**: Creation and tracking of core business entities.

### 🛰️ Satellite Microservices (Future)
Independent services that handle heavy or event-driven tasks:
- **AI Summary Service**: Separate service for processing conversation summaries (CPU/Memory intensive).
- **Webhook Engine**: System for notifying external platforms of events (Asynchronous/Retry logic).
- **Integrations**: Connectors to third-party tools.

## Development Protocol
- **Stability First**: Changes to the Core must be heavily verified and follow established [Patterns](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/knowledge/patterns.md).
- **Delegation**: If a feature doesn't require real-time persistence or is computationally expensive, evaluate if it should be a Satellite Service instead of being added to the Core.

## 🔄 Interaction Pattern
When adding a feature:
1. Define the **Model** and the **Trait** (Contract).
2. Implement the **Repository** (Infrastructure).
3. Create the **Service** (Application) that uses the repository.
4. Add the **Handler** (Interface) that calls the service.
