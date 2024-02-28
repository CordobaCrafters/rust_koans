#[cfg(test)]
mod tests {
    /// We shall contemplate truth by testing reality, via asserts
    #[test]
    fn test_assert_truth() {
        assert!(false)
    }

    /// Enlightenment may be more easily achieved with appropriate messages.
    #[test]
    fn test_assert_with_message() {
        assert!(false, "This should be True -- Please fix this")
    }

    /// Sometimes we will ask you to fill in the values.
    #[test]
    fn test_fill_in_values() {
        assert_eq!(0, 1 + 1)
    }

    /// To understand reality, we must compare our expectations against reality.
    #[test]
    fn test_assert_equality() {
        let expected_value = 0; // Update this
        let actual_value = 1 + 1;
        assert!(expected_value == actual_value)
    }

    /// Some ways of asserting equality are better than others.
    #[test]
    fn test_a_better_way_of_asserting_equality() {
        let expected_value = 0; // Update this
        let actual_value = 1 + 1;
        assert_eq!(expected_value, actual_value)
    }

    /// It's also important to ensure things are not equal when expected.
    #[test]
    fn test_assert_not_equal() {
        let expected_value = 2; // Update this
        let actual_value = 1 + 1;
        assert_ne!(
            expected_value, actual_value,
            "These values should not be equal, please fix this"
        );
    }
}
