    #[test]
    fn new_account_is_empty() {
        let acc = open_account("Ana");
        assert_eq!(balance(&acc), 0);
        assert_eq!(acc.borrow().owner, "Ana");
    }

    #[test]
    fn two_owners_see_the_same_money() {
        let a = open_account("Mat");
        let b = Rc::clone(&a);
        deposit(&a, 100);
        assert_eq!(withdraw(&b, 30), Ok(()));
        assert_eq!(balance(&a), 70);
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn cannot_withdraw_too_much() {
        let acc = open_account("Bob");
        deposit(&acc, 20);
        assert_eq!(withdraw(&acc, 50), Err("not enough money".to_string()));
        assert_eq!(balance(&acc), 20);
    }
