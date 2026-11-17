    #[test]
    fn counts_ignore_case_and_punctuation() {
        let counts = word_counts("Rust! rust, RUST? fun");
        assert_eq!(counts.get("rust"), Some(&3));
        assert_eq!(counts.get("fun"), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn lone_punctuation_is_not_a_word() {
        assert_eq!(word_counts("hi - hi").len(), 1);
    }

    #[test]
    fn top_three() {
        let text = "The cat sat. The cat ran! A dog? The end.";
        assert_eq!(
            top_words(text, 3),
            vec![("the".to_string(), 3), ("cat".to_string(), 2), ("a".to_string(), 1)]
        );
    }

    #[test]
    fn fewer_words_than_asked() {
        assert_eq!(top_words("one", 5), vec![("one".to_string(), 1)]);
        assert_eq!(top_words("", 5), vec![]);
    }
