use super::*;

pub(super) async fn run(command: AggregateCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AggregateSubcommand::Get(arg) => {
            let query = aggregate_query(&command.options)?;
            print_json(
                &client
                    .get_json_with_query::<_, Value>(
                        &format!("/api/aggregate/{}", arg.model),
                        &query,
                    )
                    .await?,
            )?
        }
        crate::cli::AggregateSubcommand::Run(arg) => {
            let body = aggregate_body(&command.options)?;
            print_json(
                &client
                    .post_json::<_, Value>(&format!("/api/aggregate/{}", arg.model), &body)
                    .await?,
            )?
        }
    }
    Ok(())
}

fn aggregate_query(options: &AggregateOptions) -> anyhow::Result<Vec<(String, String)>> {
    let mut query = Vec::new();
    if options.count {
        query.push(("count".to_string(), String::new()));
    }
    if let Some(sum) = &options.sum {
        query.push(("sum".to_string(), sum.clone()));
    }
    if let Some(avg) = &options.avg {
        query.push(("avg".to_string(), avg.clone()));
    }
    if let Some(min) = &options.min {
        query.push(("min".to_string(), min.clone()));
    }
    if let Some(max) = &options.max {
        query.push(("max".to_string(), max.clone()));
    }
    if let Some(where_source) = &options.r#where {
        let value = read_json_source(where_source)?;
        query.push(("where".to_string(), serde_json::to_string(&value)?));
    }
    Ok(query)
}

fn aggregate_body(options: &AggregateOptions) -> anyhow::Result<Value> {
    if let Some(body_source) = &options.body {
        let body = read_json_source(body_source)?;
        if !body.is_object() {
            return Err(anyhow::anyhow!("aggregate body must be a JSON object"));
        }
        return Ok(body);
    }

    let mut object = serde_json::Map::new();
    if let Some(where_source) = &options.r#where {
        object.insert("where".to_string(), read_json_source(where_source)?);
    }

    let mut aggregate = serde_json::Map::new();
    if options.count {
        aggregate.insert("count".to_string(), json!({"$count": "*"}));
    }
    if let Some(sum) = &options.sum {
        aggregate.insert("sum".to_string(), json!({"$sum": sum}));
    }
    if let Some(avg) = &options.avg {
        aggregate.insert("avg".to_string(), json!({"$avg": avg}));
    }
    if let Some(min) = &options.min {
        aggregate.insert("min".to_string(), json!({"$min": min}));
    }
    if let Some(max) = &options.max {
        aggregate.insert("max".to_string(), json!({"$max": max}));
    }
    if !aggregate.is_empty() {
        object.insert("aggregate".to_string(), Value::Object(aggregate));
    }

    if object.is_empty() {
        return Err(anyhow::anyhow!(
            "aggregate requires at least one flag or --body source"
        ));
    }

    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use super::{aggregate_body, aggregate_query};
    use crate::cli::AggregateOptions;
    use serde_json::json;

    #[test]
    fn aggregate_query_serializes_where_json() {
        let query = aggregate_query(&AggregateOptions {
            count: true,
            sum: Some("tokens".to_string()),
            avg: None,
            min: None,
            max: None,
            r#where: Some("{\"status\":\"open\"}".to_string()),
            body: None,
        })
        .expect("query should build");

        assert_eq!(
            query,
            vec![
                ("count".to_string(), String::new()),
                ("sum".to_string(), "tokens".to_string()),
                ("where".to_string(), "{\"status\":\"open\"}".to_string()),
            ]
        );
    }

    #[test]
    fn aggregate_body_builds_from_flags() {
        let body = aggregate_body(&AggregateOptions {
            count: true,
            sum: Some("tokens".to_string()),
            avg: None,
            min: None,
            max: None,
            r#where: Some("{\"status\":\"open\"}".to_string()),
            body: None,
        })
        .expect("body should build");

        assert_eq!(
            body,
            json!({
                "where": { "status": "open" },
                "aggregate": {
                    "count": { "$count": "*" },
                    "sum": { "$sum": "tokens" }
                }
            })
        );
    }

    #[test]
    fn aggregate_body_rejects_non_object_inline_body() {
        let error = aggregate_body(&AggregateOptions {
            count: false,
            sum: None,
            avg: None,
            min: None,
            max: None,
            r#where: None,
            body: Some("[1,2,3]".to_string()),
        })
        .expect_err("non-object body should fail");

        assert!(error.to_string().contains("JSON object"));
    }
}
