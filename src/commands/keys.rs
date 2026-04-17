use super::*;

pub(super) async fn run(command: KeysCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        KeysSubcommand::List => print_json(&client.get_json::<Value>("/api/keys").await?)?,
        KeysSubcommand::Create(args) => {
            let body = keys_create_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/keys", &body).await?)?;
        }
        KeysSubcommand::Rotate(args) => {
            let body = keys_rotate_body(args)?;
            print_json(
                &client
                    .post_json::<_, Value>("/api/keys/rotate", &body)
                    .await?,
            )?;
        }
        KeysSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/keys/{}", arg.key_id))
                .await?,
        )?,
    }
    Ok(())
}

fn keys_create_body(args: KeysCreateCommand) -> anyhow::Result<Value> {
    let mut object = Map::new();
    if let Some(user_id) = args.user_id {
        object.insert("user_id".to_string(), Value::String(user_id));
    }
    if let Some(public_key) = read_secret_source_option(args.public_key.as_deref())? {
        object.insert("public_key".to_string(), Value::String(public_key));
    }
    if let Some(name) = args.name {
        object.insert("name".to_string(), Value::String(name));
    }
    if let Some(algorithm) = args.algorithm {
        object.insert("algorithm".to_string(), Value::String(algorithm));
    }
    if let Some(expires_at) = args.expires_at {
        object.insert("expires_at".to_string(), Value::String(expires_at));
    }
    Ok(Value::Object(object))
}

fn keys_rotate_body(args: KeysRotateCommand) -> anyhow::Result<Value> {
    let mut object = Map::new();
    if let Some(key_id) = args.key_id {
        object.insert("key_id".to_string(), Value::String(key_id));
    }
    if let Some(new_public_key) = read_secret_source_option(args.new_public_key.as_deref())? {
        object.insert("new_public_key".to_string(), Value::String(new_public_key));
    }
    if let Some(algorithm) = args.algorithm {
        object.insert("algorithm".to_string(), Value::String(algorithm));
    }
    if let Some(new_name) = args.new_name {
        object.insert("new_name".to_string(), Value::String(new_name));
    }
    if let Some(revoke_old_after_seconds) = args.revoke_old_after_seconds {
        object.insert(
            "revoke_old_after_seconds".to_string(),
            Value::Number(revoke_old_after_seconds.into()),
        );
    }
    Ok(Value::Object(object))
}
