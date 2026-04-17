use super::*;

pub(super) async fn run(command: BulkCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::BulkSubcommand::Run => {
            let body = read_json_source_or_default(
                command.options.body.as_deref(),
                json!({"operations": []}),
            )?;
            let body = bulk_body(body)?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Create(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "create-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Update(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "update-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Delete(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "delete-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Export => print_json(
            &client
                .post_json::<_, Value>("/api/bulk/export", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
        crate::cli::BulkSubcommand::Import => print_json(
            &client
                .post_json::<_, Value>("/api/bulk/import", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
    }
    Ok(())
}

fn bulk_body(body: Value) -> anyhow::Result<Value> {
    match body {
        Value::Object(map) => {
            if map.contains_key("operations") {
                Ok(Value::Object(map))
            } else {
                Err(anyhow::anyhow!(
                    "bulk body must contain an operations array"
                ))
            }
        }
        other => Err(anyhow::anyhow!(
            "bulk body must be a JSON object, got {other}"
        )),
    }
}

fn bulk_model_body(options: &BulkOptions, model: &str, operation: &str) -> anyhow::Result<Value> {
    let body = match options.body.as_deref() {
        Some(source) => read_json_source(source)?,
        None => read_json_body_or_default(json!([]))?,
    };

    if !body.is_array() {
        return Err(anyhow::anyhow!("bulk model body must be a JSON array"));
    }

    Ok(json!({
        "operations": [
            {
                "operation": operation,
                "model": model,
                "data": body,
            }
        ]
    }))
}
