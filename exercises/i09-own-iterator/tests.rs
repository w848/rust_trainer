    #[test]
    fn first_ten() {
        let first: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn works_with_other_iterator_tools() {
        assert_eq!(Fibonacci::new().skip(20).next(), Some(6765));
    }

    #[test]
    fn even_sum() {
        assert_eq!(even_sum_below(100), 44);
        assert_eq!(even_sum_below(4_000_000), 4613732);
    }
