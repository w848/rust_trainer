    #[test]
    fn single_number() {
        assert_eq!(eval(&num(7)), 7);
        assert_eq!(to_text(&num(7)), "7");
    }

    #[test]
    fn evaluates_a_tree() {
        assert_eq!(eval(&mul(add(num(1), num(2)), num(3))), 9);
        assert_eq!(eval(&add(num(10), neg(num(4)))), 6);
    }

    #[test]
    fn writes_a_tree() {
        assert_eq!(to_text(&mul(add(num(1), num(2)), num(3))), "((1 + 2) * 3)");
        assert_eq!(to_text(&neg(add(num(1), num(2)))), "-(1 + 2)");
    }
