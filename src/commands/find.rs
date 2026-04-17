use super::*;

pub(super) async fn run(command: FindCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::FindSubcommand::Query(arg) => {
            let body = find_query_body(&command.options)?;
            print_json(
                &client
                    .post_json::<_, Value>(&format!("/api/find/{}", arg.model), &body)
                    .await?,
            )?
        }
        crate::cli::FindSubcommand::Saved(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/find/{}/{}", arg.model, arg.target))
                .await?,
        )?,
    }
    Ok(())
}

fn find_query_body(options: &FindOptions) -> anyhow::Result<Value> {
    let mut body = match options.r#where.as_deref() {
        Some(source) => read_json_source(source)?,
        None => Value::Object(serde_json::Map::new()),
    };

    let object = match &mut body {
        Value::Object(map) => map,
        Value::Null => {
            body = Value::Object(serde_json::Map::new());
            match &mut body {
                Value::Object(map) => map,
                _ => unreachable!(),
            }
        }
        other => {
            return Err(anyhow::anyhow!(
                "find where JSON must be an object, got {other}"
            ));
        }
    };

    if let Some(select) = &options.select {
        object.insert(
            "select".to_string(),
            Value::Array(
                select
                    .split(',')
                    .map(|part| part.trim())
                    .filter(|part| !part.is_empty())
                    .map(|part| Value::String(part.to_string()))
                    .collect(),
            ),
        );
    }
    if let Some(order) = &options.order {
        object.insert(
            "order".to_string(),
            Value::Array(
                order
                    .split(',')
                    .map(|part| part.trim())
                    .filter(|part| !part.is_empty())
                    .map(|part| Value::String(part.to_string()))
                    .collect(),
            ),
        );
    }
    if let Some(limit) = options.limit {
        object.insert("limit".to_string(), Value::Number(limit.into()));
    }
    if let Some(offset) = options.offset {
        object.insert("offset".to_string(), Value::Number(offset.into()));
    }

    Ok(body)
}
