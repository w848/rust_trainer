    #[test]
    fn finds_the_position() {
        assert_eq!(find_index(&[4, 8, 15, 16], 15), Some(2));
        assert_eq!(find_index(&[4, 8, 15, 16], 4), Some(0));
    }

    #[test]
    fn first_one_wins() {
        assert_eq!(find_index(&[5, 8, 8], 8), Some(1));
    }

    #[test]
    fn missing_gives_none() {
        assert_eq!(find_index(&[1, 2, 3], 9), None);
        assert_eq!(find_index(&[], 1), None);
    }
