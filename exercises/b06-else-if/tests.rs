    #[test]
    fn positive() {
        assert_eq!(sign(7), 1);
    }

    #[test]
    fn negative() {
        assert_eq!(sign(-3), -1);
    }

    #[test]
    fn zero() {
        assert_eq!(sign(0), 0);
    }
