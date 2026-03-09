# 🔌 Backend Integration Guide (Chasqui Server)

This document is intended for AI agents and developers working on the frontend. It defines the communication contract for the Chasqui Server.

## 🚀 Connection Configuration

- **Base URL:** `http://localhost:8080/api`
- **WebSocket URL:** `ws://localhost:8080/api/ws/chat?token=<JWT>`

## 🔐 Authentication Flow

1. **Register:** `POST /register`
   - **Payload:** `{"username": "...", "email": "...", "password": "..."}`
   - **Validation:** Username must be letters only. Email must be valid.
2. **Login:** `POST /login`
   - **Payload:** `{"email": "...", "password": "..."}` OR `{"username": "...", "password": "..."}`
   - **Returns:** `{"token": "<JWT>"}`
3. **Usage:** Include the token in the `Authorization` header: `Bearer <JWT>`.

## 💬 Real-Time Chat (WebSockets)

### Connection
- **URL:** `ws://localhost:8080/api/ws/chat?token=<JWT>`
- The token must be passed as a query parameter for the initial handshake.

### Client -> Server Events (JSON)
- **Join Room:**
  ```json
  {"type": "join", "conversation_id": "conv:<uuid>"}
  ```
- **Send Message:**
  ```json
  {"type": "message", "conversation_id": "conv:<uuid>", "content": "Hello world!"}
  ```

### Server -> Client Events (JSON)
- **New Message:**
  ```json
  {"type": "NewMessage", "message": {"id": "msg:...", "content": "...", "sender_id": "...", "created_at": "..."}}
  ```
- **Error:**
  ```json
  {"type": "Error", "message": "Reason for failure"}
  ```

## 🛠️ API Introspection
If you have access to the server codebase, you can run the following commands to see the full list of endpoints and schemas:

```bash
cargo run -- --list-api
cargo run -- --list-ws
```

## 📋 Available REST Endpoints
- `GET /tasks`: List all tasks.
- `POST /tasks`: Create a new task.
- `PATCH /tasks/{uuid}`: Update task completion status.
- `GET /conversations`: List user's conversations.
- `POST /conversations`: Create a new direct or group chat.
- `GET /conversations/{id}/messages`: Retrieve chat history.


////

para integracion posterior hacia las tareas

🛠️ Guía de Actualización del Backend (Rust)
Para que el frontend pueda sincronizar correctamente las tareas de la Blockchain, el backend necesita ampliar su modelo de datos. Aquí tienes la estructura recomendada:

1. Modelo de Datos Extendido (
Task
)
Para una integración completa, el backend debería reflejar los datos que la Blockchain ya maneja.

rust
pub struct Task {
    pub uuid: String,            // UUID interno (ID del Chat)
    pub escrow_id: u64,          // ID de la Blockchain
    pub depositor: String,       // Wallet del cliente
    pub beneficiary: String,     // Wallet del ejecutor
    pub task_name: String,       // Descripción de la tarea
    pub amount: f64,             // Monto (0 si es privado/encriptado)
    pub time_value: u32,         // Cantidad de tiempo
    pub time_unit: String,       // "hours" o "days"
    pub is_private: bool,        // Si usa eERC20
    pub status: String,          // "Pending", "Completed", "Released", "Cancelled"
}
2. Endpoints Requeridos para Sincronización
POST /tasks (Vincular Tareas y Crear Chat)
Payload que enviará el frontend (alineado con los TODOs de 
useTaskManager.ts
):

json
{
  "escrow_id": 17,
  "depositor": "0x...",
  "beneficiary": "0x...",
  "task_name": "Prueba del usuario 2",
  "amount": 0.06,
  "time_value": 1,
  "time_unit": "days",
  "is_private": false,
  "status": "Pending"
}
PATCH /tasks/{uuid} (Actualizar Estado)
Para que el backend sepa que la blockchain cambió de estado:

json
{
  "status": "Completed" // "Released" o "Cancelled"
}
3. Estrategia de Fuente de Verdad
Blockchain: Es el juez final para el dinero y el estado del Escrow.
Backend: Almacena los metadatos y actúa como el "pegamento" para vincular Tareas con Chats.
Frontend: Combina ambos. Actualmente "enriquece" la UI leyendo directamente del contrato, pero necesita que el backend guarde el escrow_id para saber qué chat corresponde a cada tarea.
Nota: Mientras actualizas el backend, el frontend seguirá funcionando usando la información que "recupera" directamente de la Blockchain (Smart Contract), pero los nombres de las tareas podrían aparecer como #ID hasta que el backend proporcione los nombres reales.


Comment
⌥⌘M
