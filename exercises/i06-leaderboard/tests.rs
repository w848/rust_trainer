    #[test]
    fn highest_first() {
        let ranked = rank(vec![player("a", 1), player("b", 3), player("c", 2)]);
        let scores: Vec<u32> = ranked.iter().map(|p| p.score).collect();
        assert_eq!(scores, vec![3, 2, 1]);
    }

    #[test]
    fn ties_are_alphabetical() {
        let ranked = rank(vec![player("Zoe", 50), player("Mat", 80), player("Ana", 50)]);
        assert_eq!(ranked, vec![player("Mat", 80), player("Ana", 50), player("Zoe", 50)]);
    }

    #[test]
    fn podium_of_three() {
        let players = vec![player("Zoe", 50), player("Mat", 80), player("Ana", 50), player("Bob", 10)];
        assert_eq!(podium(players), vec!["Mat", "Ana", "Zoe"]);
    }

    #[test]
    fn small_podium() {
        assert_eq!(podium(vec![player("Solo", 1)]), vec!["Solo"]);
        assert_eq!(podium(vec![]), Vec::<String>::new());
    }
