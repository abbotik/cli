pub(super) fn vec_or_none(values: Vec<String>) -> Option<Vec<String>> {
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

#[cfg(test)]
mod tests {
    use super::vec_or_none;

    #[test]
    fn vec_or_none_returns_none_for_empty_vectors() {
        assert_eq!(vec_or_none(Vec::<String>::new()), None);
    }

    #[test]
    fn vec_or_none_preserves_non_empty_vectors() {
        assert_eq!(
            vec_or_none(vec!["rooms:read".to_string(), "rooms:edit".to_string()]),
            Some(vec!["rooms:read".to_string(), "rooms:edit".to_string()])
        );
    }
}
