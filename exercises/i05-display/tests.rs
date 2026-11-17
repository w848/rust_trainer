    #[test]
    fn prints_with_zeros() {
        assert_eq!(Time::new(9, 5).to_string(), "09:05");
        assert_eq!(Time::new(23, 59).to_string(), "23:59");
        assert_eq!(format!("{}", Time::new(0, 0)), "00:00");
    }

    #[test]
    fn adds_minutes() {
        assert_eq!(Time::new(9, 5).add_minutes(70).to_string(), "10:15");
    }

    #[test]
    fn goes_around_midnight() {
        assert_eq!(Time::new(23, 50).add_minutes(20).to_string(), "00:10");
        assert_eq!(Time::new(12, 0).add_minutes(24 * 60).to_string(), "12:00");
    }
