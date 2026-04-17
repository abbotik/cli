use super::*;
use super::auth::vec_or_none;

pub(super) async fn run(command: UserCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        UserSubcommand::Me => print_json(&client.get_json::<Value>("/api/user/me").await?)?,
        UserSubcommand::List(args) => {
            let query = user_list_query(&args);
            print_json(
                &client
                    .get_json_with_query::<_, Value>("/api/user", &query)
                    .await?,
            )?
        }
        UserSubcommand::Create(args) => {
            let body = user_create_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/user", &body).await?)?
        }
        UserSubcommand::Invite(args) => {
            let request = user_invite_request(args);
            print_json(&client.user_invite(&request).await?)?;
        }
        UserSubcommand::ApiKeys(command) => match command.command {
            UserApiKeysSubcommand::List => {
                print_json(&client.get_json::<Value>("/api/user/api-keys").await?)?
            }
            UserApiKeysSubcommand::Create(args) => {
                let body = user_api_keys_create_body(args)?;
                print_json(
                    &client
                        .post_json::<_, Value>("/api/user/api-keys", &body)
                        .await?,
                )?
            }
            UserApiKeysSubcommand::Delete(arg) => print_json(
                &client
                    .delete_json::<Value>(&format!("/api/user/api-keys/{}", arg.key_id))
                    .await?,
            )?,
            UserApiKeysSubcommand::RevokeAll => print_json(
                &client
                    .post_json::<_, Value>("/api/user/api-keys/revoke-all", &json!({}))
                    .await?,
            )?,
        },
        UserSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/user/{}", arg.id))
                .await?,
        )?,
        UserSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/user/{}", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        UserSubcommand::Delete(args) => {
            let body = json!({
                "confirm": args.confirm,
                "reason": args.reason,
            });
            print_json::<Value>(
                &client
                    .request_json(
                        Method::DELETE,
                        &format!("/api/user/{}", args.id),
                        Some(&body),
                    )
                    .await?,
            )?;
        }
        UserSubcommand::Password(args) => {
            let body = user_password_body(args)?;
            print_json::<Value>(
                &client
                    .post_json::<_, Value>(&format!("/api/user/{}/password", body.id), &body.body)
                    .await?,
            )?;
        }
        UserSubcommand::Sudo(args) => print_json(&client.auth_sudo(args.reason.as_deref()).await?)?,
        UserSubcommand::Fake(args) => {
            let body = json!({
                "user_id": args.user_id,
                "username": args.username,
            });
            print_json::<Value>(
                &client
                    .post_json::<_, Value>("/api/user/fake", &body)
                    .await?,
            )?
        }
    }
    Ok(())
}

fn user_invite_request(args: UserInviteCommand) -> InviteRequest {
    InviteRequest {
        username: args.username,
        invite_type: args.invite_type,
        access: args.access,
        access_read: vec_or_none(args.access_read),
        access_edit: vec_or_none(args.access_edit),
        access_full: vec_or_none(args.access_full),
        expires_in: args.expires_in,
    }
}

fn user_list_query(args: &UserListCommand) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(limit) = args.limit {
        query.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(offset) = args.offset {
        query.push(("offset".to_string(), offset.to_string()));
    }
    query
}

fn user_create_body(args: UserCreateCommand) -> anyhow::Result<Value> {
    if let Some(body) = args.body {
        return Ok(serde_json::from_str(&body)?);
    }

    let mut object = Map::new();
    if let Some(name) = args.name {
        object.insert("name".to_string(), Value::String(name));
    }
    if let Some(auth) = args.auth {
        object.insert("auth".to_string(), Value::String(auth));
    }
    if let Some(access) = args.access {
        object.insert("access".to_string(), Value::String(access));
    }

    if object.is_empty() {
        return read_json_body_or_default(json!({}));
    }

    Ok(Value::Object(object))
}

fn user_api_keys_create_body(args: UserApiKeysCreateCommand) -> anyhow::Result<Value> {
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

struct PasswordBody {
    id: String,
    body: Value,
}

fn user_password_body(args: UserPasswordCommand) -> anyhow::Result<PasswordBody> {
    let mut object = Map::new();
    if let Some(current_password) = args.current_password {
        object.insert(
            "current_password".to_string(),
            Value::String(current_password),
        );
    }
    if let Some(new_password) = args.new_password {
        object.insert("new_password".to_string(), Value::String(new_password));
    }

    if object.is_empty() {
        return Ok(PasswordBody {
            id: args.id,
            body: read_json_body_or_default(json!({}))?,
        });
    }

    Ok(PasswordBody {
        id: args.id,
        body: Value::Object(object),
    })
}
