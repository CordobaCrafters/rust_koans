#[cfg(test)]
mod tests {
    fn truth_value(condition: bool) -> &'static str {
        if condition {
            "true stuff"
        } else {
            "false stuff"
        }
    }

    #[test]
    fn test_true_is_treated_as_true() {
        assert_eq!("change me", truth_value(true))
    }

    #[test]
    fn test_false_is_treated_as_false() {
        assert_eq!("change me", truth_value(false))
    }
}
