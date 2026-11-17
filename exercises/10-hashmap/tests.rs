    #[test]
    fn counts_repeated_words() {
        let counts = word_count("the cat and the hat");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts.len(), 4);
    }

    #[test]
    fn ignores_case() {
        let counts = word_count("Rust rust RUST");
        assert_eq!(counts["rust"], 3);
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn extra_spaces_and_lines() {
        let counts = word_count("  one\n two   one ");
        assert_eq!(counts["one"], 2);
        assert_eq!(counts["two"], 1);
    }

    #[test]
    fn empty_text() {
        assert!(word_count("").is_empty());
    }
