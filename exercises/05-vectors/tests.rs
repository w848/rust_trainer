    #[test]
    fn keeps_even_numbers_in_order() {
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn negative_and_zero() {
        assert_eq!(evens(&[-3, -2, 0, 7]), vec![-2, 0]);
    }

    #[test]
    fn nothing_to_keep() {
        assert_eq!(evens(&[1, 3, 5]), Vec::<i32>::new());
        assert_eq!(evens(&[]), Vec::<i32>::new());
    }
