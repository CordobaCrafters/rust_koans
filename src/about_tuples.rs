#[cfg(test)]
mod tests {
    #[test]
    fn test_create_a_tuple() {
        let expected = (5, 5);
        assert_eq!(expected, (0, 0))
    }

    #[test]
    fn test_access_elements_of_tuple_by_index() {
        let tuple = (1, 2);
        assert_eq!(0, tuple.0);
        assert_eq!(0, tuple.1);
    }

    #[test]
    fn test_tuples_can_have_plenty_of_elements() {
        let tuple = (4, 1, 3, 3, 7, 8, 7);
        assert_eq!(0, tuple.5);
    }

    #[test]
    fn test_tuples_can_be_tuple_members() {
        let tuple = (0, 1, 2, 3, 4, (5, 6));
        assert_eq!(0, tuple.5 .0);
    }

    #[test]
    fn test_can_mix_types() {
        let tuple = (1, "Hello!");
        assert_eq!("", tuple.1);
    }

    #[test]
    fn test_can_create_tuples_with_empty_elements() {
        let tuple = (5,);

        assert_eq!((0,), tuple);
    }

    #[test]
    fn test_tuples_are_good_for_representing_records() {
        let locations = (
            ("Nashville", (36.1636, 86.7823)),
            ("New York", (40.7128, 74.0060)),
        );

        assert_eq!("", locations.0 .0);
    }

    #[test]
    fn test_tuples_can_be_destructured() {
        let tuple = (1, 2, 3);
        let (a, b, c) = tuple;

        assert_eq!(0, a);
        assert_eq!(0, b);
        assert_eq!(0, c);
    }
}
