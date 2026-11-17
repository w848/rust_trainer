    #[test]
    fn four() {
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn small_ones() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn ten() {
        assert_eq!(factorial(10), 3628800);
    }
