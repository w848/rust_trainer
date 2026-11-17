    #[test]
    fn small_numbers() {
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(4), 10);
    }

    #[test]
    fn zero() {
        assert_eq!(sum_to(0), 0);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(sum_to(100), 5050);
    }
