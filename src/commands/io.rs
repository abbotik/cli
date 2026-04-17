use super::*;

pub(super) fn read_stdin_or_empty() -> anyhow::Result<String> {
    if stdio::stdin().is_terminal() {
        return Ok(String::new());
    }

    let mut buffer = String::new();
    let mut stdin = stdio::stdin();
    if stdin.read_to_string(&mut buffer).is_ok() && !buffer.trim().is_empty() {
        return Ok(buffer);
    }
    Ok(String::new())
}

pub(super) fn read_json_body_or_default(default: Value) -> anyhow::Result<Value> {
    let raw = read_stdin_or_empty()?;
    if raw.trim().is_empty() {
        return Ok(default);
    }

    Ok(serde_json::from_str(&raw)?)
}

pub(super) fn read_json_source_or_default(source: Option<&str>, default: Value) -> anyhow::Result<Value> {
    match source {
        Some(source) if !source.is_empty() => read_json_source(source),
        _ => Ok(default),
    }
}

pub(super) fn read_json_source(source: &str) -> anyhow::Result<Value> {
    let raw = if source == "-" {
        read_stdin_or_empty()?
    } else if let Some(path) = source.strip_prefix('@') {
        stdfs::read_to_string(path)?
    } else {
        source.to_string()
    };

    if raw.trim().is_empty() {
        return Ok(Value::Null);
    }

    Ok(serde_json::from_str(&raw)?)
}

pub(super) fn read_secret_source_option(source: Option<&str>) -> anyhow::Result<Option<String>> {
    source.map(read_secret_source).transpose()
}

pub(super) fn read_secret_source(source: &str) -> anyhow::Result<String> {
    let raw = if source == "-" {
        read_stdin_or_empty()?
    } else if let Some(path) = source.strip_prefix('@') {
        stdfs::read_to_string(path)?
    } else {
        source.to_string()
    };

    Ok(trim_one_trailing_newline(raw))
}

fn trim_one_trailing_newline(mut value: String) -> String {
    if value.ends_with('\n') {
        value.pop();
        if value.ends_with('\r') {
            value.pop();
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::{read_json_source, read_secret_source, trim_one_trailing_newline};

    #[test]
    fn trim_one_trailing_newline_removes_single_lf_or_crlf() {
        assert_eq!(trim_one_trailing_newline("token\n".to_string()), "token");
        assert_eq!(trim_one_trailing_newline("token\r\n".to_string()), "token");
        assert_eq!(trim_one_trailing_newline("token".to_string()), "token");
        assert_eq!(trim_one_trailing_newline("token\n\n".to_string()), "token\n");
    }

    #[test]
    fn read_json_source_parses_inline_json() {
        let value = read_json_source("{\"ok\":true}").expect("inline json should parse");
        assert_eq!(value["ok"], true);
    }

    #[test]
    fn read_secret_source_trims_one_trailing_newline_from_inline_value() {
        let value = read_secret_source("secret\n").expect("inline secret should parse");
        assert_eq!(value, "secret");
    }
}
