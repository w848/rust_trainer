    #[test]
    fn adults() {
        assert_eq!(is_adult(18), true);
        assert_eq!(is_adult(45), true);
    }

    #[test]
    fn children() {
        assert_eq!(is_adult(17), false);
        assert_eq!(is_adult(0), false);
    }
