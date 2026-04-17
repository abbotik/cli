use super::*;

pub(super) async fn run(command: KeysCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        KeysSubcommand::List => print_json(&client.get_json::<Value>("/api/keys").await?)?,
        KeysSubcommand::Create(args) => {
            let body = keys_create_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/keys", &body).await?)?;
        }
        KeysSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/keys/{}", arg.key_id))
                .await?,
        )?,
        KeysSubcommand::RevokeAll => print_json(
            &client
                .post_json::<_, Value>("/api/keys/revoke-all", &json!({}))
                .await?,
        )?,
    }
    Ok(())
}

fn keys_create_body(args: KeysCreateCommand) -> anyhow::Result<Value> {
    if let Some(body) = args.body {
        return Ok(serde_json::from_str(&body)?);
    }

    let mut object = Map::new();
    if let Some(name) = args.name {
        object.insert("name".to_string(), Value::String(name));
    }
    if let Some(expires_at) = args.expires_at {
        object.insert("expires_at".to_string(), Value::String(expires_at));
    }

    if object.is_empty() {
        return read_json_body_or_default(json!({}));
    }

    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use super::keys_create_body;
    use crate::cli::KeysCreateCommand;
    use serde_json::json;

    #[test]
    fn keys_create_body_prefers_inline_json() {
        let body = keys_create_body(KeysCreateCommand {
            body: Some("{\"name\":\"ci\"}".to_string()),
            name: None,
            expires_at: None,
        })
        .expect("body should parse");

        assert_eq!(body, json!({"name": "ci"}));
    }

    #[test]
    fn keys_create_body_builds_object_from_flags() {
        let body = keys_create_body(KeysCreateCommand {
            body: None,
            name: Some("CI runner".to_string()),
            expires_at: Some("2026-12-31T23:59:59Z".to_string()),
        })
        .expect("body should build");

        assert_eq!(
            body,
            json!({
                "name": "CI runner",
                "expires_at": "2026-12-31T23:59:59Z"
            })
        );
    }
}
