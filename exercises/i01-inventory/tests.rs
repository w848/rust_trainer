    #[test]
    fn total_of_the_shop() {
        let shop = vec![item("apple", 0.5, 40), item("bread", 2.0, 5), item("milk", 1.25, 12)];
        assert_eq!(total_value(&shop), 45.0);
    }

    #[test]
    fn empty_shop() {
        assert_eq!(total_value(&[]), 0.0);
        assert_eq!(most_stocked(&[]), None);
    }

    #[test]
    fn finds_most_stocked() {
        let shop = vec![item("bread", 2.0, 5), item("apple", 0.5, 40), item("milk", 1.25, 12)];
        assert_eq!(most_stocked(&shop), Some("apple"));
    }
