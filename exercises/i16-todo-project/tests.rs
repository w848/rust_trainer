    #[test]
    fn adds_with_increasing_ids() {
        let mut list = TodoList::new();
        assert_eq!(list.add("a"), 1);
        assert_eq!(list.add("b"), 2);
        assert_eq!(list.pending(), vec!["a", "b"]);
    }

    #[test]
    fn completes_tasks() {
        let mut list = TodoList::new();
        let a = list.add("a");
        list.add("b");
        assert_eq!(list.complete(a), Ok(()));
        assert_eq!(list.pending(), vec!["b"]);
        assert_eq!(list.summary(), "1 done, 1 to do");
    }

    #[test]
    fn unknown_id_is_an_error() {
        let mut list = TodoList::new();
        assert_eq!(list.complete(7), Err("no task with id 7".to_string()));
    }

    #[test]
    fn removes_and_never_reuses_ids() {
        let mut list = TodoList::new();
        let a = list.add("a");
        list.add("b");
        let removed = list.remove(a).unwrap();
        assert_eq!(removed.title, "a");
        assert_eq!(list.remove(a), None);
        assert_eq!(list.add("c"), 3);
        assert_eq!(list.summary(), "0 done, 2 to do");
    }
