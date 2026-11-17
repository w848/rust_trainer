    #[test]
    fn small_list() {
        assert_eq!(sum_of_odd_squares(&[1, 2, 3]), 10);
        assert_eq!(sum_of_odd_squares(&[1, 2, 3, 4, 5]), 35);
    }

    #[test]
    fn negative_odd_numbers_count() {
        assert_eq!(sum_of_odd_squares(&[-3, 2]), 9);
    }

    #[test]
    fn no_odd_numbers() {
        assert_eq!(sum_of_odd_squares(&[2, 4, 6]), 0);
        assert_eq!(sum_of_odd_squares(&[]), 0);
    }
