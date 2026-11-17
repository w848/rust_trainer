    #[test]
    fn capitalizes_one_word() {
        assert_eq!(capitalize("rUST"), "Rust");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn title_case_sentence() {
        assert_eq!(title_case("the rust book"), "The Rust Book");
    }

    #[test]
    fn extra_spaces_disappear() {
        assert_eq!(title_case("  hello    WORLD "), "Hello World");
        assert_eq!(title_case("   "), "");
    }
