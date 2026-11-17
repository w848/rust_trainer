    #[test]
    fn sums_a_million() {
        let numbers: Vec<u64> = (1..=1_000_000).collect();
        assert_eq!(parallel_sum(numbers, 4), 500_000_500_000);
    }

    #[test]
    fn uneven_parts_and_one_worker() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4, 5, 6, 7], 3), 28);
        assert_eq!(parallel_sum(vec![5, 5], 1), 10);
        assert_eq!(parallel_sum(vec![], 4), 0);
    }

    #[test]
    fn squares_through_a_channel() {
        assert_eq!(squares(5), vec![1, 4, 9, 16, 25]);
        assert_eq!(squares(0), vec![]);
    }
