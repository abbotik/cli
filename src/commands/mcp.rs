use super::*;

const MCP_PROTOCOL_VERSION: &str = "2025-03-26";

pub(super) async fn run(command: McpCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        McpSubcommand::List(_) => mcp_list(client).await?,
        McpSubcommand::Call(command) => mcp_call(command, client).await?,
    }
    Ok(())
}

async fn mcp_list(client: &ApiClient) -> anyhow::Result<()> {
    let session = initialize_session(client).await?;
    let response = mcp_request(
        client,
        "tools/list",
        json!({}),
        2,
        Some(&session.session_id),
        Some(&session.protocol_version),
    )
    .await?;
    print_json(&response)?;
    Ok(())
}

async fn mcp_call(command: McpCallCommand, client: &ApiClient) -> anyhow::Result<()> {
    let arguments = read_json_source_or_default(command.arguments.as_deref(), json!({}))?;
    let session = initialize_session(client).await?;
    let response = mcp_request(
        client,
        "tools/call",
        json!({
            "name": command.tool,
            "arguments": arguments,
        }),
        2,
        Some(&session.session_id),
        Some(&session.protocol_version),
    )
    .await?;
    print_json(&response)?;
    Ok(())
}

struct McpSession {
    session_id: String,
    protocol_version: String,
}

async fn initialize_session(client: &ApiClient) -> anyhow::Result<McpSession> {
    let (response, headers) = mcp_request_with_response_headers(
        client,
        "initialize",
        json!({
            "protocolVersion": MCP_PROTOCOL_VERSION,
            "clientInfo": {
                "name": "abbot",
                "version": env!("CARGO_PKG_VERSION"),
            },
        }),
        1,
        None,
        None,
    )
    .await?;

    let session_id = headers
        .get("Mcp-Session-Id")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| anyhow::anyhow!("MCP initialize response did not include a session id"))?
        .to_string();
    let protocol_version = headers
        .get("MCP-Protocol-Version")
        .and_then(|value| value.to_str().ok())
        .or_else(|| {
            response
                .get("result")
                .and_then(|result| result.get("protocolVersion"))
                .and_then(Value::as_str)
        })
        .unwrap_or(MCP_PROTOCOL_VERSION)
        .to_string();

    Ok(McpSession {
        session_id,
        protocol_version,
    })
}

async fn mcp_request(
    client: &ApiClient,
    method: &str,
    params: Value,
    id: u64,
    session_id: Option<&str>,
    protocol_version: Option<&str>,
) -> anyhow::Result<Value> {
    let mut headers = vec![
        (
            "Accept".to_string(),
            "application/json, text/event-stream".to_string(),
        ),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];
    if let Some(session_id) = session_id {
        headers.push(("Mcp-Session-Id".to_string(), session_id.to_string()));
    }
    if let Some(protocol_version) = protocol_version {
        headers.push((
            "MCP-Protocol-Version".to_string(),
            protocol_version.to_string(),
        ));
    }

    let body = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    });

    Ok(client
        .request_json_with_headers::<_, Value>(Method::POST, "/mcp", Some(&headers), Some(&body))
        .await?)
}

async fn mcp_request_with_response_headers(
    client: &ApiClient,
    method: &str,
    params: Value,
    id: u64,
    session_id: Option<&str>,
    protocol_version: Option<&str>,
) -> anyhow::Result<(Value, reqwest::header::HeaderMap)> {
    let mut headers = vec![
        (
            "Accept".to_string(),
            "application/json, text/event-stream".to_string(),
        ),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];
    if let Some(session_id) = session_id {
        headers.push(("Mcp-Session-Id".to_string(), session_id.to_string()));
    }
    if let Some(protocol_version) = protocol_version {
        headers.push((
            "MCP-Protocol-Version".to_string(),
            protocol_version.to_string(),
        ));
    }

    let body = json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    });

    Ok(client
        .request_json_with_headers_and_response_headers::<_, Value>(
            Method::POST,
            "/mcp",
            Some(&headers),
            Some(&body),
        )
        .await?)
}
