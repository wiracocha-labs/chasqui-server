//! API Documentation module
//! Provides functions to list and describe available API endpoints.

/// Prints all registered API routes and their expected JSON payloads.
pub fn print_routes() {
    println!("\n🚀 Chasqui Server - API Endpoints Documentation\n");
    println!("{:<10} {:<40} {:<30}", "METHOD", "PATH", "DESCRIPTION");
    println!("{}", "-".repeat(80));

    print_endpoint(
        "GET",
        "/api/tasks",
        "Retrieve all tasks",
        None,
        Some(r#"[{"uuid": "string", "task_name": "string"}]"#),
    );

    print_endpoint(
        "POST",
        "/api/tasks",
        "Create a new task",
        Some(r#"{"task_name": "string"}"#),
        Some(r#"{"uuid": "string", "task_name": "string"}"#),
    );

    print_endpoint(
        "PATCH",
        "/api/tasks/{uuid}",
        "Update task status",
        None,
        Some(r#"{"uuid": "string", "task_name": "string"}"#),
    );

    print_endpoint(
        "POST",
        "/api/register",
        "Register a new user",
        Some(
            r#"{"username": "Alice", "email": "alice@example.com", "password": "..."} OR {"wallet": "0x..."}"#,
        ),
        Some(r#"{"create": "success", "message": "User created successfully"}"#),
    );

    print_endpoint(
        "POST",
        "/api/login",
        "Authenticate and get JWT token",
        Some(
            r#"{"email": "...", "password": "..."} OR {"username": "...", "password": "..."} OR {"wallet": "0x..."}"#,
        ),
        Some(r#"{"token": "<JWT_STRING>"}"#),
    );

    print_endpoint(
        "GET",
        "/api/users",
        "Retrieve all users",
        None,
        Some(r#"[{"username": "...", "email": "...", "wallet": "0x..."}]"#),
    );

    print_endpoint(
        "DELETE",
        "/api/users/wallets",
        "Delete all users with wallets",
        None,
        Some(r#"{"create": "success", "message": "..."}"#),
    );

    print_endpoint(
        "GET",
        "/api/ws/chat",
        "WebSocket chat connection (requires token in query)",
        None,
        Some(r#"ws://localhost:8080/api/ws/chat?token=<JWT>"#),
    );

    print_endpoint(
        "POST",
        "/api/conversations",
        "Create a new conversation",
        Some(
            r#"{"target_wallet": "0x...", "conversation_type": "Direct"} OR {"participant_ids": ["uuid"], "conversation_type": "Group", "name": "..."}"#,
        ),
        Some(
            r#"{"id": "conversation:uuid", "conversation_type": "Direct|Group", "participants": ["user:uuid"]}"#,
        ),
    );

    print_endpoint(
        "GET",
        "/api/conversations",
        "List user conversations",
        None,
        Some(
            r#"[{"id": "conversation:uuid", "conversation_type": "Direct|Group", "participants": ["user:uuid"]}]"#,
        ),
    );

    print_endpoint(
        "GET",
        "/api/conversations/{id}/messages",
        "Get message history (query: limit, offset)",
        None,
        Some(
            r#"[{"id": "msg:uuid", "content": "...", "sender_id": "user:uuid", "conversation_id": "conversation:uuid", "created_at": "..."}]"#,
        ),
    );

    print_endpoint(
        "POST",
        "/api/conversations/{id}/participants",
        "Add participant to conversation",
        Some(r#"{"identifier": "0x... or user:uuid"}"#),
        Some(r#"{"status": "success"}"#),
    );

    println!("\n💡 Tips:");
    println!("- Use Bearer token in 'Authorization' header for protected routes");
    println!("- Conversation IDs format: 'conversation:uuid'");
    println!("- User IDs format: 'user:uuid'");
    println!("- Message IDs format: 'msg:uuid'");
    println!("- Wallet authentication supported (no password required)\n");
}

/// Prints documentation for WebSocket messages and events.
pub fn print_ws_docs() {
    println!("\n🌐 Chasqui Server - WebSocket Documentation\n");
    println!("Connection URL: /api/ws/chat?token=<JWT>");
    println!("Note: The 'token' query parameter is required for the initial handshake.\n");

    println!("--- CLIENT -> SERVER MESSAGES ---");
    println!("Sent by the client to the server.\n");

    print_ws_message(
        "join",
        "Join a conversation room to receive messages",
        r#"{"type": "join", "conversation_id": "conversation:uuid"}"#,
    );

    print_ws_message(
        "message",
        "Send a new message to a conversation",
        r#"{"type": "message", "conversation_id": "conversation:uuid", "content": "Hello!"}"#,
    );

    println!("\n--- SERVER -> CLIENT MESSAGES ---");
    println!("Sent by the server to one or more clients.\n");

    print_ws_message(
        "NewMessage",
        "Broadcast when a new message is saved",
        r#"{"type": "NewMessage", "message": {"id": "msg:uuid", "content": "...", "sender_id": "user:uuid", "created_at": "..."}}"#,
    );

    print_ws_message(
        "Error",
        "Sent when an action fails",
        r#"{"type": "Error", "message": "Description of the error"}"#,
    );

    println!("💡 Tip: All messages are JSON strings.\n");
}

fn print_ws_message(msg_type: &str, desc: &str, example: &str) {
    println!("{:<15} - {}", msg_type, desc);
    println!("  PAYLOAD: {}", example);
    println!();
}

fn print_endpoint(method: &str, path: &str, desc: &str, req: Option<&str>, res: Option<&str>) {
    println!("{:<10} {:<40} {:<30}", method, path, desc);
    if let Some(r) = req {
        println!("  REQUEST JSON:  {}", r);
    }
    if let Some(r) = res {
        println!("  RESPONSE JSON: {}", r);
    }
    println!();
}
