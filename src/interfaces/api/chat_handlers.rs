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
    pub participant_ids: Vec<String>,
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
) -> HttpResponse {
    let creator_id = match extract_user_id(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let participants: Vec<Thing> = body
        .participant_ids
        .iter()
        .map(|id| {
            let parts: Vec<&str> = id.split(':').collect();
            if parts.len() == 2 {
                Thing::from((parts[0], parts[1]))
            } else {
                Thing::from(("user", id.as_str()))
            }
        })
        .collect();

    let result = match body.conversation_type {
        ConversationType::Direct => {
            if participants.len() != 2 {
                return HttpResponse::BadRequest().body("Direct chat requires 2 participants");
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
        Ok(conv) => HttpResponse::Ok().json(conv),
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
