mod tests {
    #[test]
    fn test_a_result_can_be_ok() {
        let result: Result<i32, bool> = Ok(5);
        assert_eq!(false, result.is_ok());
    }

    #[test]
    fn test_a_result_can_be_err() {
        let result: Result<i32, bool> = Err(false);
        assert_eq!(false, result.is_err());
    }

    #[test]
    fn test_ok_can_be_unwrapped() {
        let result: Result<i32, bool> = Ok(5);
        assert_eq!(0, result.unwrap());
    }

    #[test]
    #[should_panic(expected = "Fail to unwrap Err")]
    fn test_err_cannot_be_unwrapped() {
        let result: Result<i32, bool> = Err(false);
        assert_eq!(0, result.expect("CHANGE ME"));
    }

    #[test]
    fn test_err_can_be_unwrapped_with_default_value() {
        let result: Result<i32, bool> = Err(false);
        assert_eq!(0, result.unwrap_or(5))
    }

    #[test]
    fn test_err_can_be_unwrapped_with_computed_value() {
        let bool_to_i32 = |x| if x { 1 } else { 0 };
        let result: Result<i32, bool> = Err(true);
        assert_eq!(0, result.unwrap_or_else(bool_to_i32))
    }

    #[test]
    fn test_result_can_use_pattern_matching() {
        let result: Result<i32, bool> = Ok(5);

        let value = match result {
            Ok(number) => number,
            Err(x) => {
                if x {
                    1
                } else {
                    0
                }
            }
        };

        assert_eq!(0, value);
    }

    #[test]
    fn test_all_variants_must_be_used_to_compile() {
        let mut _enabled = false;
        #[cfg(remove_this_line)]
        {
            let result: Result<i32, bool> = Err(true);

            let value: i32 = match result {
                Err(x) => {
                    if x {
                        1
                    } else {
                        0
                    }
                }
            };

            assert_eq!(0, value);

            _enabled = true;
        }
        assert!(_enabled, "This test intentionally fails.");
    }

    #[test]
    fn test_if_let_ignores_err_result_alternative() {
        let result: Result<i32, bool> = Ok(5);
        let mut value = 0;

        if let Ok(number) = result {
            value = number;
        }
        assert_eq!(0, value);
    }

    #[test]
    fn test_ok_result_can_be_concatenated() {
        let result: Result<i32, bool> = Ok(5);
        let value = result.and_then(|x| Ok(x * 2));
        assert_eq!(Ok(5), value);
    }

    #[test]
    fn test_err_result_cannot_be_concatenated() {
        let result: Result<i32, bool> = Err(false);
        let value = result.and_then(|x| Ok(x * 2));
        assert_eq!(Ok(5), value);
    }

    #[test]
    fn test_ok_result_can_be_mapped() {
        let result: Result<i32, bool> = Ok(5);
        let value = result.map(|x| x * 2);
        assert_eq!(Ok(5), value);
    }

    #[test]
    fn test_err_result_cannot_be_mapped() {
        let result: Result<i32, bool> = Err(false);
        let value = result.map(|x| x * 2);
        assert_eq!(Ok(5), value);
    }
}
