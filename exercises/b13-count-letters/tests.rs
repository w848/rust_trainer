    #[test]
    fn banana() {
        assert_eq!(count_letter("banana", 'a'), 3);
    }

    #[test]
    fn not_there() {
        assert_eq!(count_letter("rust", 'z'), 0);
    }

    #[test]
    fn capitals_are_different() {
        assert_eq!(count_letter("Anna", 'a'), 1);
    }
