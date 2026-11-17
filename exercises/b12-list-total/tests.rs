    #[test]
    fn one_two_three() {
        assert_eq!(total(&[1, 2, 3]), 6);
    }

    #[test]
    fn empty_list() {
        assert_eq!(total(&[]), 0);
    }

    #[test]
    fn with_negatives() {
        assert_eq!(total(&[10, -4, 6]), 12);
    }
