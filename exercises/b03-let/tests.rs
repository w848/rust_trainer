    #[test]
    fn two_hours() {
        assert_eq!(hours_to_minutes(2), 120);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(hours_to_minutes(0), 0);
        assert_eq!(hours_to_minutes(1), 60);
    }
