#[cfg(test)]
mod tests {
    use crate::type_of;

    #[test]
    fn test_double_quoted_strings_are_str() {
        let string = "Hello, world!";
        assert_eq!("", type_of(string), "&str are the borrowed form of str");
    }

    #[test]
    fn test_single_quoted_strings_are_char() {
        let char = 'c';
        assert_eq!("", type_of(char));
    }

    #[test]
    fn test_raw_strings_are_also_str() {
        let a = r#"Konnichi wa, world!"#;
        let b = "Konnichi wa, world!";
        assert_eq!(false, a == b);
    }

    #[test]
    fn test_use_backslash_for_escaping_quotes_in_str() {
        let a = "He said, \"Don't\"";
        let b = r#"He said, "Don't""#;
        assert_eq!(false, a == b);
    }

    #[test]
    fn test_str_can_span_lines() {
        let a = "Howdy,
world!";
        let b = "Howdy,\nworld!";
        assert_eq!(false, a == b);
    }

    #[test]
    fn test_str_has_attributes() {
        let string = "Hello, 
world!";
        assert_eq!(0, string.len());
        assert_eq!(true, string.is_empty());
    }

    #[test]
    fn test_test_you_can_get_a_slice_from_a_str() {
        let string = "Hello, world!";
        assert_eq!(Some(""), string.get(0..5));
        assert_eq!(Some(""), string.get(..5));
        assert_eq!(Some(""), string.get(7..));
    }

    #[test]
    fn test_str_can_be_split() {
        let string = "Hello, world!";
        let (first, last) = string.split_at(6);

        assert_eq!("", first);
        assert_eq!("", last);
    }

    #[test]
    fn test_strings_can_be_split_with_different_patterns() {
        let string = "Hello, world!";
        let mut iter = string.split(", ");
        assert_eq!(Some(""), iter.next());
        assert_eq!(Some(""), iter.next());
        assert_eq!(None, iter.next());
    }
}
