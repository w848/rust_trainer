    #[test]
    fn two_plus_three() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn works_with_zero_and_negatives() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-4, 10), 6);
    }
