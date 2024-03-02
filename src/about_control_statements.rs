#[cfg(test)]
mod tests {
    #[test]
    fn test_if_then_statements() {
        let mut value: bool = false;
        if true {
            value = true;
        }

        assert_eq!(false, value);
    }

    #[test]
    fn test_if_then_else_statements() {
        let value: bool;
        if true {
            value = true;
        } else {
            value = false;
        }

        assert_eq!(false, value);
    }

    #[test]
    fn test_if_statements_return_values() {
        let value = if true { true } else { false };
        assert_eq!(false, value);

        let value = if false { true } else { false };
        assert_eq!(true, value);
    }

    #[test]
    fn test_if_statements_with_no_else_does_not_compile() {
        let mut _enabled = false;
        // To activate that kind of koan, delete the line containing the [cfg] command.
        #[cfg(remove_this_line)]
        {
            let value = if true {
                true
            };

            assert_eq!(true, value);

            _enabled = true;
        }
        assert!(_enabled, "This test intentionally fails.");
    }

    #[test]
    fn test_loop_creates_infinite_loop() {
        let mut count = 0;
        let mut alternative_count = 0;
        loop {
            count += 1;

            if count == 2 {
                // Skip the rest of this iteration
                continue;
            }

            alternative_count += 1;

            if count == 3 {
                // Exit this loop
                break;
            }
        }

        assert_eq!(0, count);
        assert_eq!(0, alternative_count);
    }

    #[test]
    #[allow(unreachable_code, unused_labels)]
    fn test_loops_can_be_nested() {
        let mut _value = 0;
        'outer: loop {
            'inner: loop {
                _value = 1;

                // Labels allow us to get out of outer loops.
                break 'outer;
            }

            _value = 2;
        }

        assert_eq!(0, _value);
    }

    #[test]
    fn test_loop_statements_return_values() {
        let mut count = 0;
        let value = loop {
            count += 1;
            if count == 3 {
                break count;
            }
        };

        assert_eq!(0, value);
    }

    #[test]
    fn test_while_statements() {
        let mut i = 1;
        let mut value = 1;
        while i <= 10 {
            value = value * i;
            i += 1;
        }

        assert_eq!(0, value);
    }

    #[test]
    fn test_break_statement() {
        let mut i = 1;
        let mut value = 1;
        while i <= 1000 {
            if i > 10 {
                break;
            }

            value = value * i;
            i += 1;
        }

        assert_eq!(0, value);
    }

    #[test]
    fn test_continue_statement() {
        let mut i = 0;
        let mut value = vec![];
        while i < 10 {
            i += 1;
            if i % 2 == 0 {
                continue;
            }
            value.push(i);
        }

        assert_eq!(vec![0], value);
    }

    #[test]
    fn test_for_statement_with_ranges() {
        let mut sum = 0;
        for i in 1..10 {
            sum += i;
        }

        assert_eq!(0, sum);
    }

    #[test]
    fn test_for_statement_with_iterators() {
        let words = vec!["fish", "and", "chips"];
        let mut result: Vec<String> = vec![];
        for &word in words.iter() {
            result.push(word.to_uppercase());
        }

        assert_eq!(vec!["fish", "and", "chips"], result);
    }
}
