//! Chat API Handlers
//!
//! Handles HTTP requests related to chat and upgrades connections to WebSocket.

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use surrealdb::sql::Thing;

use crate::application::services::conversation_service::ConversationService;
use crate::application::services::message_service::MessageService;
use crate::infrastructure::auth::jwt::validate_token;
use crate::infrastructure::websocket::chat_server::ChatServer;
use crate::infrastructure::websocket::session::WsSession;
use crate::models::entities::conversation::ConversationType;

/// DTO for creating a new conversation
#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    /// List of participant IDs (optional if target_wallet is provided)
    pub participant_ids: Option<Vec<String>>,
    /// Optional wallet address to create a chat with
    pub target_wallet: Option<String>,
    pub conversation_type: ConversationType,
    pub name: Option<String>,
}

/// DTO for get messages pagination
#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Helper to extract user_id from Authorization header or Query param
fn extract_user_id(req: &HttpRequest) -> Option<Thing> {
    // 1. Try Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    // 2. Try query parameter (fallback for WebSockets)
    let token = token.or_else(|| {
        req.query_string()
            .split('&')
            .find(|s| s.starts_with("token="))
            .and_then(|s| s.strip_prefix("token="))
            .map(|s| s.to_string())
    });

    token
        .and_then(|t| validate_token(&t).ok())
        .map(|claims| Thing::from(("user", claims.sub.as_str())))
}

/// Handlers for WebSocket connection
///
/// Upgrades the HTTP connection to WebSocket and starts a WsSession actor.
/// For testing, we are using a dummy user_id.
pub async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    ws::start(WsSession::new(user_id, srv.get_ref().clone()), &req, stream)
}

/// POST /api/conversations
pub async fn create_conversation(
    req: HttpRequest,
    body: web::Json<CreateConversationRequest>,
    conversation_service: web::Data<ConversationService>,
    db: web::Data<crate::infrastructure::database::surrealdb::Database>,
) -> HttpResponse {
    use crate::models::traits::user_data_trait::UserDataTrait;

    let creator_id = match extract_user_id(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut participant_ids = Vec::new();

    // Priority 1: target_wallet (shorthand for direct chat)
    if let Some(wallet) = &body.target_wallet {
        participant_ids.push(wallet.to_lowercase());
    } else if let Some(ids) = &body.participant_ids {
        participant_ids = ids.into_iter().map(|id| id.to_lowercase()).collect();
    } else {
        return HttpResponse::BadRequest()
            .body("Either 'participant_ids' or 'target_wallet' must be provided");
    }

    let mut participants: Vec<Thing> = Vec::new();
    // Always include creator
    participants.push(creator_id.clone());

    for identifier in &participant_ids {
        // Resolve identifier to User ID
        let resolved_id = if identifier.contains(':') {
            // Already a Thing format (user:id)
            let p: Vec<&str> = identifier.split(':').collect();
            Some(Thing::from((p[0], p[1])))
        } else if uuid::Uuid::parse_str(identifier).is_ok() {
            // Raw UUID
            Some(Thing::from(("user", identifier.as_str())))
        } else {
            // Treat as wallet address
            match db.find_user_by_wallet(identifier).await {
                Some(user) => user.id,
                None => {
                    return HttpResponse::BadRequest()
                        .body(format!("User with wallet {} not found", identifier));
                }
            }
        };

        if let Some(id) = resolved_id {
            if !participants.contains(&id) {
                participants.push(id);
            }
        }
    }

    let result = match body.conversation_type {
        ConversationType::Direct => {
            if participants.len() != 2 {
                return HttpResponse::BadRequest()
                    .body("Direct chat requires exactly 2 participants (including you)");
            }
            conversation_service
                .create_direct_conversation(participants[0].clone(), participants[1].clone())
                .await
        }
        ConversationType::Group => {
            conversation_service
                .create_group_conversation(
                    body.name.clone().unwrap_or_else(|| "New Group".to_string()),
                    creator_id,
                    participants,
                )
                .await
        }
    };

    match result {
        Ok(conversation) => HttpResponse::Ok().json(conversation),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// GET /api/conversations
pub async fn get_conversations(
    req: HttpRequest,
    conversation_service: web::Data<ConversationService>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match conversation_service.get_user_conversations(user_id).await {
        Ok(convs) => HttpResponse::Ok().json(convs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// GET /api/conversations/{id}/messages
pub async fn get_messages(
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<GetMessagesQuery>,
    message_service: web::Data<MessageService>,
) -> HttpResponse {
    if extract_user_id(&req).is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let conv_id_str = path.into_inner();
    let parts: Vec<&str> = conv_id_str.split(':').collect();
    if parts.len() != 2 {
        return HttpResponse::BadRequest().body("Invalid conversation ID format");
    }

    let conv_id = Thing::from((parts[0], parts[1]));
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    match message_service
        .get_conversation_history(conv_id, limit, offset)
        .await
    {
        Ok(msgs) => HttpResponse::Ok().json(msgs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// POST /api/conversations/{id}/participants
pub async fn add_participant(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<serde_json::Value>,
    conversation_service: web::Data<ConversationService>,
    db: web::Data<crate::infrastructure::database::surrealdb::Database>,
) -> HttpResponse {
    use crate::models::traits::user_data_trait::UserDataTrait;

    if extract_user_id(&req).is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let conv_id_str = path.into_inner();
    let parts: Vec<&str> = conv_id_str.split(':').collect();
    if parts.len() != 2 {
        return HttpResponse::BadRequest().body("Invalid conversation ID format");
    }
    let conv_id = Thing::from((parts[0], parts[1]));

    let identifier = match body.get("identifier").and_then(|v| v.as_str()) {
        Some(id) => id.to_lowercase(),
        None => {
            return HttpResponse::BadRequest()
                .body("Missing 'identifier' field (wallet or user ID)")
        }
    };

    // Resolve identifier to User ID
    let target_user_id = if identifier.contains(':') {
        let p: Vec<&str> = identifier.split(':').collect();
        Thing::from((p[0], p[1]))
    } else if uuid::Uuid::parse_str(&identifier).is_ok() {
        Thing::from(("user", identifier.as_str()))
    } else {
        match db.find_user_by_wallet(&identifier).await {
            Some(user) => user.id.expect("User has no ID"),
            None => return HttpResponse::BadRequest().body("User with wallet not found"),
        }
    };

    match conversation_service
        .add_participant(conv_id, target_user_id)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
