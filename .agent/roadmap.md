# 🗺️ Chasqui Server - Roadmap

Este documento guía el desarrollo futuro del servidor de chat.

## 🔜 Próximas Mejoras (Corto Plazo)
1.  **Sincronización Inicial:** Evento `{"type": "sync"}` para recuperar chats y mensajes al conectar.
2.  **Búsqueda Automática:** Iniciar chats enviando solo el `user_id` del destinatario (el servidor resuelve si la sala existe).
3.  **Presencia:** Estados de "Online" y "Escribiendo...".

## 🚀 Visión a Largo Plazo
*   Escalabilidad horizontal mediante Actix y Redis (opcional).
*   Cifrado de extremo a extremo en el campo `content`.
*   Sistema de roles dinámico persistido en SurrealDB.

> [!NOTE]
> Ver `future_improvements.md` en la carpeta de cerebro para una lista exhaustiva de tareas.
