use crate::cli::DataOptions;

pub fn query_pairs(options: &DataOptions) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if options.include_trashed {
        query.push(("include_trashed".to_string(), "true".to_string()));
    }
    if options.include_deleted {
        query.push(("include_deleted".to_string(), "true".to_string()));
    }
    if options.unwrap {
        query.push(("unwrap".to_string(), "true".to_string()));
    }
    if let Some(select) = &options.select {
        query.push(("select".to_string(), select.clone()));
    }
    if let Some(where_filter) = &options.r#where {
        query.push(("where".to_string(), where_filter.clone()));
    }
    if let Some(limit) = options.limit {
        query.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(stat) = options.stat {
        query.push(("stat".to_string(), stat.to_string()));
    }
    if let Some(access) = options.access {
        query.push(("access".to_string(), access.to_string()));
    }
    if options.permanent {
        query.push(("permanent".to_string(), "true".to_string()));
    }
    if options.upsert {
        query.push(("upsert".to_string(), "true".to_string()));
    }
    query
}

#[cfg(test)]
mod tests {
    use super::query_pairs;
    use crate::cli::DataOptions;

    #[test]
    fn includes_limit_when_present() {
        let options = DataOptions {
            limit: Some(10),
            ..DataOptions::default()
        };

        let query = query_pairs(&options);
        assert!(query.contains(&("limit".to_string(), "10".to_string())));
    }
}
