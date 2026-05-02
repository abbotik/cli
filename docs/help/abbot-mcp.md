# MCP

List and call tools exposed by Abbotik's MCP endpoint.

This command is MCP-shaped rather than HTTP-shaped. It does not expose raw
`GET /mcp`, `POST /mcp`, SSE, or legacy message routes.

Examples:

```bash
abbot mcp list
abbot mcp call abbot_data --arguments '{"action":"list","model":"rooms"}'
abbot mcp call abbot_auth --arguments @auth-call.json
```
