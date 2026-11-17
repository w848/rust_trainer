    #[test]
    fn several_words() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn single_word() {
        assert_eq!(first_word("rust"), "rust");
    }

    #[test]
    fn leading_spaces_are_ignored() {
        assert_eq!(first_word("   crab life"), "crab");
    }

    #[test]
    fn empty_text() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn works_on_a_borrowed_string() {
        let owned = String::from("borrow me");
        assert_eq!(first_word(&owned), "borrow");
        assert_eq!(owned, "borrow me"); // still ours
    }
