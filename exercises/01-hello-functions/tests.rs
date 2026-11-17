    #[test]
    fn greets_ferris() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
    }

    #[test]
    fn greets_anyone() {
        assert_eq!(greet("Mat"), "Hello, Mat!");
        assert_eq!(greet(""), "Hello, !");
    }
