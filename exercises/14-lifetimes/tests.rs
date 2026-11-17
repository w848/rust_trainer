    #[test]
    fn first_is_longer() {
        assert_eq!(longest("hello world", "hi"), "hello world");
    }

    #[test]
    fn second_is_longer() {
        assert_eq!(longest("hi", "hello"), "hello");
    }

    #[test]
    fn same_length_gives_the_first() {
        assert_eq!(longest("abc", "xyz"), "abc");
    }

    #[test]
    fn works_with_owned_strings() {
        let a = String::from("ferris");
        let b = String::from("crab");
        assert_eq!(longest(&a, &b), "ferris");
    }
