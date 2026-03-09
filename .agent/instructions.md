# 🤖 Chasqui Server - AI Instructions

Estas son las reglas de oro y decisiones arquitectónicas que deben seguirse para mantener la consistencia del proyecto.

## 🛠️ Protocolo de Interacción (CRÍTICO)
*   **Sistema de Skills:** Antes de realizar tareas complejas (como añadir endpoints o migrar lógica), busca si existe una guía detallada en `.agent/skills/<tarea>/SKILL.md` y síguela estrictamente.
*   **Análisis Previo:** ANTES de editar cualquier archivo, el agente debe realizar un análisis exhaustivo del código afectado.
*   **Comunicación:** El agente debe reportar primero bugs, mejoras e impacto esperado.
*   **Aprobación:** Solo se debe proceder una vez que el usuario haya validado la propuesta.

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
