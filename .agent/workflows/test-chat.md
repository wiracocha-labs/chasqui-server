---
description: Flujo completo de prueba para el sistema de chat (Auth + WebSocket)
---

# Test Chat Workflow

Este workflow verifica que el sistema de chat sea funcional de extremo a extremo.

### 1. Iniciar el Servidor
// turbo
```bash
cargo run
```

### 2. Obtener Token de Acceso
```bash
# Cambia las credenciales según tu usuario
curl -X POST http://localhost:8080/api/login \
-H "Content-Type: application/json" \
-d '{
  "username": "renzodosprueba",
  "password": "TU_PASSWORD"
}'
```

### 3. Verificar Salas Existentes
```bash
curl -H "Authorization: Bearer <TOKEN>" http://localhost:8080/api/conversations
```

### 4. Conectar WebSocket y Probar
```bash
# Conectar
websocat -v "ws://localhost:8080/api/ws/chat?token=<TOKEN>"

# En la terminal de websocat, enviar:
# {"type": "join", "conversation_id": "conversation:ID_ENCONTRADO_EN_PASO_3"}
# {"type": "message", "conversation_id": "conversation:ID_ENCONTRADO_EN_PASO_3", "content": "Prueba Workflow"}
```
