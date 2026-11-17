    #[test]
    fn sums_only_numbers() {
        assert_eq!(sum_valid(&["4", " 6 ", "seven", "-2"]), 8);
        assert_eq!(sum_valid(&["nope"]), 0);
    }

    #[test]
    fn dot() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
        assert_eq!(dot_product(&[1, 2, 3], &[10]), 10);
    }

    #[test]
    fn positions() {
        assert_eq!(positions_of(&["a", "b", "a", "c", "a"], "a"), vec![0, 2, 4]);
        assert_eq!(positions_of(&["a"], "z"), vec![]);
    }
