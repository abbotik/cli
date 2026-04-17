use super::*;

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

#[cfg(test)]
mod tests {
    use super::{
        user_api_keys_create_body, user_create_body, user_invite_request, user_list_query,
        user_password_body, PasswordBody,
    };
    use crate::cli::{
        UserApiKeysCreateCommand, UserCreateCommand, UserInviteCommand, UserListCommand,
        UserPasswordCommand,
    };
    use serde_json::json;

    #[test]
    fn user_invite_request_omits_empty_access_lists() {
        let request = user_invite_request(UserInviteCommand {
            username: Some("alice".to_string()),
            invite_type: Some("human".to_string()),
            access: Some("full".to_string()),
            access_read: Vec::new(),
            access_edit: vec!["rooms:edit".to_string()],
            access_full: Vec::new(),
            expires_in: Some(3600),
        });

        assert_eq!(request.username.as_deref(), Some("alice"));
        assert_eq!(request.access_read, None);
        assert_eq!(request.access_edit, Some(vec!["rooms:edit".to_string()]));
        assert_eq!(request.access_full, None);
    }

    #[test]
    fn user_list_query_includes_limit_and_offset() {
        let query = user_list_query(&UserListCommand {
            limit: Some(10),
            offset: Some(20),
        });
        assert_eq!(
            query,
            vec![
                ("limit".to_string(), "10".to_string()),
                ("offset".to_string(), "20".to_string())
            ]
        );
    }

    #[test]
    fn user_create_body_prefers_inline_json() {
        let body = user_create_body(UserCreateCommand {
            body: Some("{\"name\":\"alice\"}".to_string()),
            name: None,
            auth: None,
            access: None,
        })
        .expect("body should parse");

        assert_eq!(body, json!({"name": "alice"}));
    }

    #[test]
    fn user_api_keys_create_body_builds_object_from_flags() {
        let body = user_api_keys_create_body(UserApiKeysCreateCommand {
            body: None,
            name: Some("bot".to_string()),
            expires_at: Some("2026-04-20T00:00:00Z".to_string()),
        })
        .expect("body should build");

        assert_eq!(
            body,
            json!({
                "name": "bot",
                "expires_at": "2026-04-20T00:00:00Z"
            })
        );
    }

    #[test]
    fn user_password_body_uses_flag_values_when_present() {
        let PasswordBody { id, body } = user_password_body(UserPasswordCommand {
            id: "user_123".to_string(),
            current_password: Some("old".to_string()),
            new_password: Some("new".to_string()),
        })
        .expect("body should build");

        assert_eq!(id, "user_123");
        assert_eq!(
            body,
            json!({
                "current_password": "old",
                "new_password": "new"
            })
        );
    }
}
