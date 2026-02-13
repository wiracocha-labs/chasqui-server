# 🤖 Chasqui Server - AI Instructions

Estas son las reglas de oro y decisiones arquitectónicas que deben seguirse para mantener la consistencia del proyecto.

## 🔑 Gestión de Identidades (IDs)
*   **Formato de IDs:** Usamos `surrealdb::sql::Thing`. Sin embargo, al exponer IDs en JWT o APIs externas, se deben extraer como **UUIDs limpios** (sin los brackets `⟨ ⟩` de SurrealDB).
*   **Enlace de Datos:** En repositorios, usar `.bind(("id", thing))` en queries nativas en lugar de `.select((tb, id))`. Esto evita errores de conversión de strings con caracteres especiales.

## 🔐 Autenticación y JWT
*   **Claims:** El campo `sub` del JWT debe contener el UUID puro del usuario.
*   **Validación:** Siempre usar `validate_token` definido en `infrastructure/auth/jwt.rs`.

## 🌐 WebSocket & Real-Time
*   **Feedback de Errores:** Todas las acciones de WebSocket que puedan fallar deben devolver un JSON de tipo `Error`: `{"type": "Error", "message": "Descripción"}`.
*   **Comunicación:** El `ChatServer` es la fuente de verdad para la retransmisión. Los mensajes deben persistirse en SurrealDB antes de ser difundidos.

## 📂 Convenciones de Código
*   **Logging:** Usar la macro `info!`, `debug!`, y `error!` de la crate `log`.
*   **Modelos:** Los modelos deben incluir validación básica mediante el método `.is_valid()`.
