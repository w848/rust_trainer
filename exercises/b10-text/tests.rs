    #[test]
    fn hello() {
        assert_eq!(shout("hello"), "HELLO");
    }

    #[test]
    fn already_loud_and_mixed() {
        assert_eq!(shout("RUST"), "RUST");
        assert_eq!(shout("Mat 42"), "MAT 42");
    }
