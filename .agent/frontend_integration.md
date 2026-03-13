# 🔌 Backend Integration Guide (Chasqui Server)

This document is intended for AI agents and developers working on the frontend. It defines the communication contract for the Chasqui Server.

## 🚀 Connection Configuration

- **Base URL:** `http://localhost:8080/api`
- **WebSocket URL:** `ws://localhost:8080/api/ws/chat?token=<JWT>`

## 🔐 Authentication Flow

1. **Register:** `POST /register`
   - **Payload (Traditional):** `{"username": "...", "email": "...", "password": "..."}`
   - **Payload (Wallet):** `{"wallet": "0x1234...abcd"}`
   - **Validation:** Username must be letters only. Email must be valid. Wallet must be non-empty.
   - **Returns:** `{"create": "success", "message": "User created successfully"}`

2. **Login:** `POST /login`
   - **Payload (Traditional):** 
     ```json
     {"email": "...", "password": "..."} 
     // OR
     {"username": "...", "password": "..."}
     ```
   - **Payload (Wallet):** `{"wallet": "0x1234...abcd"}`
   - **Returns:** `{"token": "<JWT>"}`

3. **Usage:** Include the token in the `Authorization` header: `Bearer <JWT>`.

## 🎯 Wallet Authentication (Web3)
For Web3 integration, users can authenticate using their wallet address:

### Wallet Registration
```bash
POST /api/register
{
  "wallet": "0x1234567890abcdef1234567890abcdef12345678"
}
```

### Wallet Login (No Password Required)
```bash
POST /api/login
{
  "wallet": "0x1234567890abcdef1234567890abcdef12345678"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Notes:**
- Wallet users don't need passwords
- System generates automatic username: `wallet_0x1234_uuid`
- JWT token works the same as traditional authentication

## 💬 Real-Time Chat (WebSockets)

### Connection
- **URL:** `ws://localhost:8080/api/ws/chat?token=<JWT>`
- The token must be passed as a query parameter for the initial handshake.

### Client -> Server Events (JSON)
- **Join Room:**
  ```json
  {"type": "join", "conversation_id": "conversation:<uuid>"}
  ```
- **Send Message:**
  ```json
  {"type": "message", "conversation_id": "conversation:<uuid>", "content": "Hello world!"}
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

## 🔗 API Introspection
If you have access to the server codebase, you can run the following commands to see the full list of endpoints and schemas:

```bash
cargo run -- --list-api
cargo run -- --list-ws
```

## 📋 Available REST Endpoints

### Tasks Management
- `GET /api/tasks`: List all tasks.
  - **Response:** `[{"uuid": "...", "task_name": "..."}]`
- `POST /api/tasks`: Create a new task.
  - **Payload:** `{"task_name": "Task description"}`
  - **Response:** `{"uuid": "...", "task_name": "..."}`
- `PATCH /api/tasks/{uuid}`: Update task completion status.
  - **Response:** Updated task object

### Conversations & Chat
- `GET /api/conversations`: List user's conversations.
  - **Response:** Array of conversation objects with IDs in format `conversation:<uuid>`
- `POST /api/conversations`: Create a new chat.
  - **Direct Chat:** `{"target_wallet": "0x...", "conversation_type": "Direct"}`
  - **Group Chat:** `{"participant_ids": ["uuid", "wallet"], "conversation_type": "Group", "name": "Group Name"}`
  - **Note:** Creator is automatically included as participant
- `GET /api/conversations/{id}/messages`: Retrieve chat history.
  - **Query Params:** `?limit=50&offset=0`
  - **ID Format:** `conversation:<uuid>`
  - **Response:** Array of messages with pagination
- `POST /api/conversations/{id}/participants`: Add participant by wallet or ID.
  - **Payload:** `{"identifier": "0x... or user:uuid"}`
  - **ID Format:** `conversation:<uuid>`

## 🧠 Business Logic Details

### Authentication Flow
1. **Traditional Registration**: Requires `username`, `email`, `password`
   - Username validation: letters only
   - Email validation: basic format check
   - Password: hashed with bcrypt
2. **Wallet Registration**: Only requires `wallet` address
   - Auto-generates username: `wallet_0x1234_uuid`
   - No password required
3. **Login Methods**:
   - Traditional: `email/username + password`
   - Wallet: `wallet` only (creates user if missing)
4. **JWT Token**: Contains `sub` (user ID), `username`, `roles`, `exp`, `iat`

### Task Management
- **Current Model**: Simple structure with `uuid` and `task_name` only
- **Creation**: Auto-generates UUID, validates task_name presence
- **Updates**: Currently supports status changes via PATCH
- **Note**: Blockchain integration fields mentioned in planning are not yet implemented

### Chat System
- **Conversation Types**: `Direct` (2 participants) or `Group` (2+ participants)
- **ID Formats**: 
  - Conversations: `conversation:<uuid>`
  - Users: `user:<uuid>`
  - Messages: `msg:<uuid>`
- **Participant Resolution**: Supports wallet addresses, UUIDs, or Thing format
- **WebSocket Events**: Real-time messaging with JSON protocol

## 🔧 Implementation Notes

### Error Handling
- **400 Bad Request**: Validation errors, missing fields
- **401 Unauthorized**: Invalid credentials, missing token
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Database or system errors

### Database Schema
- **Users**: Stores traditional and wallet-based users
- **Conversations**: Links participants with conversation metadata
- **Messages**: Stores chat history with timestamps
- **Tasks**: Simple task tracking (currently minimal)

### Security Features
- JWT-based authentication
- Password hashing with bcrypt
- Token validation for all protected endpoints
- WebSocket authentication via query parameter
