    #[test]
    fn valid_ages() {
        assert_eq!(parse_age("42"), Ok(42));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn spaces_are_ignored() {
        assert_eq!(parse_age("  7 "), Ok(7));
    }

    #[test]
    fn not_a_number() {
        assert_eq!(parse_age("abc"), Err("not a number".to_string()));
        assert_eq!(parse_age(""), Err("not a number".to_string()));
        assert_eq!(parse_age("-3"), Err("not a number".to_string()));
        assert_eq!(parse_age("999"), Err("not a number".to_string()));
    }

    #[test]
    fn too_old() {
        assert_eq!(parse_age("151"), Err("too old".to_string()));
        assert_eq!(parse_age("200"), Err("too old".to_string()));
    }
