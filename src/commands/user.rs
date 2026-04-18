use super::*;

pub(super) async fn run(command: UserCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        UserSubcommand::Me(_) => print_json(&client.get_json::<Value>("/api/user/me").await?)?,
        UserSubcommand::Introspect(_) => {
            print_json(&client.get_json::<Value>("/api/user/introspect").await?)?
        }
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
        UserSubcommand::MachineKeys(command) => match command.command {
            UserMachineKeysSubcommand::List => {
                print_json(&client.get_json::<Value>("/api/user/machine-keys").await?)?
            }
            UserMachineKeysSubcommand::Create(args) => {
                let body = user_machine_keys_create_body(args)?;
                print_json(
                    &client
                        .post_json::<_, Value>("/api/user/machine-keys", &body)
                        .await?,
                )?
            }
            UserMachineKeysSubcommand::Rotate(args) => {
                let body = user_machine_keys_rotate_body(args)?;
                print_json(
                    &client
                        .post_json::<_, Value>("/api/user/machine-keys/rotate", &body)
                        .await?,
                )?
            }
            UserMachineKeysSubcommand::Delete(arg) => print_json(
                &client
                    .delete_json::<Value>(&format!("/api/user/machine-keys/{}", arg.key_id))
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

fn user_machine_keys_create_body(args: UserMachineKeysCreateCommand) -> anyhow::Result<Value> {
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

fn user_machine_keys_rotate_body(args: UserMachineKeysRotateCommand) -> anyhow::Result<Value> {
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
        user_create_body, user_invite_request, user_list_query, user_machine_keys_create_body,
        user_machine_keys_rotate_body, user_password_body, PasswordBody,
    };
    use crate::cli::{
        UserCreateCommand, UserInviteCommand, UserListCommand, UserMachineKeysCreateCommand,
        UserMachineKeysRotateCommand, UserPasswordCommand,
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
    fn user_machine_keys_create_body_builds_object_from_flags() {
        let body = user_machine_keys_create_body(UserMachineKeysCreateCommand {
            user_id: Some("user_123".to_string()),
            public_key: Some("ssh-ed25519 AAAA".to_string()),
            name: Some("bot".to_string()),
            algorithm: Some("ed25519".to_string()),
            expires_at: Some("2026-04-20T00:00:00Z".to_string()),
        })
        .expect("body should build");

        assert_eq!(
            body,
            json!({
                "user_id": "user_123",
                "public_key": "ssh-ed25519 AAAA",
                "name": "bot",
                "algorithm": "ed25519",
                "expires_at": "2026-04-20T00:00:00Z"
            })
        );
    }

    #[test]
    fn user_machine_keys_rotate_body_builds_object_from_flags() {
        let body = user_machine_keys_rotate_body(UserMachineKeysRotateCommand {
            key_id: Some("key_123".to_string()),
            new_public_key: Some("ssh-ed25519 BBBB".to_string()),
            algorithm: Some("ed25519".to_string()),
            new_name: Some("next".to_string()),
            revoke_old_after_seconds: Some(300),
        })
        .expect("body should build");

        assert_eq!(
            body,
            json!({
                "key_id": "key_123",
                "new_public_key": "ssh-ed25519 BBBB",
                "algorithm": "ed25519",
                "new_name": "next",
                "revoke_old_after_seconds": 300
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
