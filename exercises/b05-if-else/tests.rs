    #[test]
    fn second_is_bigger() {
        assert_eq!(bigger(3, 8), 8);
    }

    #[test]
    fn first_is_bigger() {
        assert_eq!(bigger(10, -2), 10);
    }

    #[test]
    fn same_numbers() {
        assert_eq!(bigger(5, 5), 5);
    }
