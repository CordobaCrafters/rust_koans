mod tests {
    // In some koans, you will have to fix a syntax error that prevents the
    // compiler from running the test. That's why they appear disabled by the
    // [cfg] order, which makes the compiler ignore the code.
    #[test]
    fn test_enable_broken_code() {
        let mut _enabled = false;
        // To activate that kind of koan, delete the line containing the [cfg] command.
        #[cfg(remove_this_line)]
        {
            let this_is_true: bool = 1;

            assert_eq!(true, this_is_true);

            _enabled = true;
        }
        assert!(_enabled, "This test intentionally fails.");
    }
}
