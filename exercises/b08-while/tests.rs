    #[test]
    fn up_to_four() {
        assert_eq!(sum_to(4), 10);
    }

    #[test]
    fn up_to_one_hundred() {
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn zero_gives_zero() {
        assert_eq!(sum_to(0), 0);
    }
