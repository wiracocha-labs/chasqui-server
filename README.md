# 🦅 Chasqui Server

> **High-performance, secure, and real-time backend engine built with Rust.**

Chasqui Server is a state-of-the-art backend framework designed for real-time communication and secure data management. Built on the power of **Actix-web** and **SurrealDB**, it provides a robust foundation for modern web applications.

---

## ✨ Core Features

*   🚀 **High Performance:** Powered by Actix-web 4.x for blazing-fast request handling.
*   💬 **Real-Time Messaging:** Native WebSocket integration for instant communication.
*   🔐 **Secure by Design:** JWT Authentication with robust Bcrypt password hashing.
*   🗃️ **Surreal Database:** Seamless integration with SurrealDB for flexible, ACID-compliant storage.
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

---

## 📊 Project Status & Roadmap

| Module | Status | Next Milestone |
| :--- | :---: | :--- |
| **Auth System** | ✅ Stable | Refresh Tokens |
| **Real-Time Chat** | ✅ 1.0 | Presence & Typing indicators |
| **Data Persistence**| ✅ Stable | Migrations & Seeds |
| **User Roles** | 🚧 Beta | Dynamic RBAC |

### 🗺️ What's Next?
*   **Hybrid Sync:** Automated message synchronization on reconnection.
*   **Media Support:** Phased rollout for image and attachment management.
*   **Scalability:** Implementation of distributed actors for horizontal growth.

---

## 🤖 For Developers & AI Agents
Detailed technical strategies, AI workflows, and architectural decisions are located in the [`.agent/`](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/) directory. 
If you are contributing using an AI assistant, please direct it to [`.agent/instructions.md`](file:///Users/renzotincopa/Documents/Wiracocha/chasqui-server/.agent/instructions.md) first.

---

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.