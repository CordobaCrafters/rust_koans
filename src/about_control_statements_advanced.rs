#[cfg(test)]
mod tests {
    #[test]
    fn test_if_let_statement() {
        fn check_optional_status(status: Option<&str>) -> String {
            if let Some(state) = status {
                format!("Status is available: {}", state)
            } else {
                "Status is unavailable".to_string()
            }
        }

        assert_eq!("", check_optional_status(Some("Active")));
        assert_eq!("", check_optional_status(None));
    }

    #[test]
    fn test_let_else_statement() {
        fn process_input(input: Option<i32>) -> String {
            let Some(value) = input else {
                return "No value provided".to_string();
            };

            format!("Value provided: {}", value)
        }

        assert_eq!("", process_input(Some(42)));
        assert_eq!("", process_input(None));
    }

    #[test]
    fn test_while_let_statement() {
        fn sum_numbers(mut numbers: Vec<Option<i32>>) -> i32 {
            let mut sum = 0;

            // Pop removes from the end and return an Option (Some or None when no more elements)
            // When Some, like numbers contains Option too, it will be Some(Some(i32)) or Some(None)
            while let Some(Some(number)) = numbers.pop() {
                println!("Number {}", number);
                sum += number;
            }

            sum
        }

        let numbers = vec![Some(3), Some(7), None, Some(2), Some(5)];

        assert_eq!(0, sum_numbers(numbers));
    }
}
