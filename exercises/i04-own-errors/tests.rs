    #[test]
    fn good_line() {
        assert_eq!(parse_setting("volume = 42"), Ok(("volume".to_string(), 42)));
        assert_eq!(parse_setting("x=-3"), Ok(("x".to_string(), -3)));
    }

    #[test]
    fn errors() {
        assert_eq!(parse_setting("   "), Err(ParseError::Empty));
        assert_eq!(parse_setting("volume 42"), Err(ParseError::MissingEquals));
        assert_eq!(parse_setting("volume = loud"), Err(ParseError::BadNumber("loud".to_string())));
    }

    #[test]
    fn whole_file() {
        assert_eq!(
            parse_all("width=800\n\nheight = 600\n"),
            Ok(vec![("width".to_string(), 800), ("height".to_string(), 600)])
        );
    }

    #[test]
    fn whole_file_stops_at_first_error() {
        assert_eq!(parse_all("a=1\nb\nc=x"), Err(ParseError::MissingEquals));
    }
