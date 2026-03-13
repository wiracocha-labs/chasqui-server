# 🦅 Chasqui Server

> **High-performance, secure, and real-time backend engine built with Rust.**

Chasqui Server is a state-of-the-art backend framework designed for real-time communication and secure data management. Built on the power of **Actix-web** and **SurrealDB**, it provides a robust foundation for modern web applications.

---

## ✨ Core Features

*   🚀 **High Performance:** Powered by Actix-web 4.x for blazing-fast request handling.
*   💬 **Real-Time Messaging:** Native WebSocket integration for instant communication.
*   🔐 **Secure & Web3 Ready:** JWT Authentication and **Wallet-based Auth** for modern decentralized integration.
*   🗃️ **Surreal Database:** Seamless integration with SurrealDB for flexible, ACID-compliant storage.
*   🏗️ **Hybrid Architecture:** Combining REST APIs with real-time WebSocket events for an optimal, hybrid sync approach.
*   📝 **Proactive Observability:** Structured logging and comprehensive error handling.
*   🛡️ **Type-Safe Validation:** Full data integrity using the `validator` crate.

---

## 🏗️ Project Architecture

```text
src/
├── application/    # Business logic & Services
├── infrastructure/ # Database & WebSocket implementations
├── interfaces/    # API Controllers & Handlers
├── models/        # Entities & Data structures
├── config/        # Environment configurations
├── error/         # Application error handling
├── lib.rs         # Local library definition
└── main.rs        # Application Entry Point
```

---

## 🚀 Getting Started

### 1. Requirements
*   Rust 1.75+
*   SurrealDB (Local or Cloud instance)

### 2. Setup
```bash
# Clone the repository
git clone https://github.com/wiracocha-labs/chasqui-server.git
cd chasqui-server

# Configure environment
cp .env.example .env
# Edit .env with your SurrealDB and Secret Key details
```

### 3. Launch
```bash
cargo run
```

### 4. Interactive Documentation
Chasqui Server includes built-in documentation for its API and WebSockets:

```bash
# List all REST API endpoints and JSON schemas (includes Wallet Auth and Wallet user routes)
cargo run -- --list-api

# List all WebSocket message types and structures
cargo run -- --list-ws
```

---

## 📊 Project Status & Roadmap

| Module | Status | Next Milestone |
| :--- | :---: | :--- |
| **Auth System** | ✅ Stable | Wallet integration & Refresh Tokens |
| **Real-Time Chat** | ✅ 1.0 | Presence & Typing indicators |
| **Data Persistence**| ✅ Stable | Migrations & Seeds |
| **User Roles** | 🚧 Beta | Dynamic RBAC |

### 🗺️ What's Next?
*   **Media Support:** Phased rollout for image and attachment management.
*   **Scalability:** Implementation of distributed actors for horizontal growth.
*   **Task Notifications:** Sending automatic, contextual notifications to the task's corresponding chat.
*   **GitHub Webhook:** Integration for remote task updates from code pushes.
*   **LLM Integration:** Basic AI summaries of tasks and conversations via LLM.

---

## 🤖 For Developers & AI Agents
Detailed technical strategies, AI workflows, and architectural decisions are located in the [`.agent/`](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/) directory. 
If you are contributing using an AI assistant, please direct it to [`.agent/instructions.md`](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/instructions.md) first.

---

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.