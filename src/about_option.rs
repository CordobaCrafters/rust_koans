#[cfg(test)]
mod tests {
    #[test]
    fn test_an_option_can_be_some() {
        let option: Option<i32> = Some(5);
        assert_eq!(false, option.is_some());
    }

    #[test]
    fn test_an_option_can_be_none() {
        let option: Option<i32> = None;
        assert_eq!(false, option.is_none());
    }

    #[test]
    fn test_some_can_be_unwrapped() {
        let option: Option<i32> = Some(5);
        assert_eq!(0, option.unwrap())
    }

    #[test]
    #[should_panic(expected = "Fail to unwrap None")]
    fn test_none_cannot_be_unwrapped() {
        let option: Option<i32> = None;
        assert_eq!(10, option.expect("CHANGE ME"));
    }

    #[test]
    fn test_none_can_be_unwrapped_with_default_value() {
        let option: Option<i32> = None;
        assert_eq!(0, option.unwrap_or(5))
    }

    #[test]
    fn test_none_can_be_unwrapped_with_computed_value() {
        let f = || 5;
        let option: Option<i32> = None;
        assert_eq!(0, option.unwrap_or_else(f))
    }

    #[test]
    fn test_option_can_use_pattern_matching() {
        let option: Option<i32> = Some(5);

        let value = match option {
            Some(number) => number,
            None => 0,
        };

        assert_eq!(0, value);
    }

    #[test]
    fn test_all_variantes_must_be_used_to_compile() {
        let mut _enabled = false;
        #[cfg(remove_this_line)]
        {
            let option: Option<i32> = None;

            let value: i32 = match option {
                None => 0,
            };

            assert_eq!(0, value);

            _enabled = true;
        }
        assert!(_enabled, "This test intentionally fails.");
    }

    #[test]
    fn test_if_let_ignores_none_option_alternative() {
        let option: Option<i32> = Some(5);
        let mut value = 0;

        if let Some(number) = option {
            value = number;
        }
        assert_eq!(0, value);
    }

    #[test]
    fn test_some_option_can_be_concatenated() {
        let option: Option<i32> = Some(5);
        let value = option.and_then(|x| Some(x * 2));
        assert_eq!(Some(5), value);
    }

    #[test]
    fn test_none_option_cannot_be_concatenated() {
        let option: Option<i32> = None;
        let value = option.and_then(|x| Some(x * 2));
        assert_eq!(Some(5), value);
    }

    #[test]
    fn test_some_option_can_be_mapped() {
        let option: Option<i32> = Some(5);
        let value = option.map(|x| x * 2);
        assert_eq!(Some(5), value);
    }

    #[test]
    fn test_none_option_cannot_be_mapped() {
        let option: Option<i32> = None;
        let value = option.map(|x| x * 2);
        assert_eq!(Some(5), value);
    }
}
