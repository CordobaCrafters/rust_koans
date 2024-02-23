#[cfg(test)]
mod tests {
    #[test]
    fn test_double_quoted_strings_are_strings() {
        let string = "Hello, world";
        assert_eq!("", string)
    }

    #[test]
    fn test_raw_strings_are_also_strings() {
        let a = r#"Konnichi wa, world!"#;
        let b = "Konnichi wa, world!";
        assert_eq!(false, a == b)
    }

    #[test]
    fn test_use_backslash_for_escaping_quotes_in_strings() {
        let a = "He said, \"Don't\"";
        let b = r#"He said, "Don't""#;
        assert_eq!(false, a == b)
    }

    #[test]
    fn test_strings_can_span_lines() {
        let a = "Howdy,
world!";
        let b = "Howdy,\nworld!";
        assert_eq!(false, a == b)
    }
}
