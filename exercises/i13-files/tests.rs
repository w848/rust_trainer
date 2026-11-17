    fn temp_file(name: &str, content: &str) -> String {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, content).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn counts_lines() {
        let path = temp_file("rust_trainer_i13_count.txt", "a\nb\nc\n");
        assert_eq!(count_lines(&path).unwrap(), 3);
    }

    #[test]
    fn finds_longest_line() {
        let path = temp_file("rust_trainer_i13_long.txt", "short\nthe longest line\nmid line\n");
        assert_eq!(longest_line(&path).unwrap(), Some("the longest line".to_string()));
    }

    #[test]
    fn empty_file() {
        let path = temp_file("rust_trainer_i13_empty.txt", "");
        assert_eq!(count_lines(&path).unwrap(), 0);
        assert_eq!(longest_line(&path).unwrap(), None);
    }

    #[test]
    fn missing_file_is_an_error() {
        assert!(count_lines("this/file/does/not/exist.txt").is_err());
        assert!(longest_line("this/file/does/not/exist.txt").is_err());
    }
