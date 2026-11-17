    #[test]
    fn stack_of_numbers() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        assert_eq!(s.len(), 2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn stack_of_strings() {
        let mut s: Stack<String> = Stack::new();
        s.push("hello".to_string());
        assert_eq!(s.peek().map(|x| x.as_str()), Some("hello"));
    }

    #[test]
    fn balanced_brackets() {
        assert!(balanced("{[()]}"));
        assert!(balanced("fn main() { let v = vec![1]; }"));
        assert!(balanced(""));
    }

    #[test]
    fn unbalanced_brackets() {
        assert!(!balanced("([)]"));
        assert!(!balanced("(("));
        assert!(!balanced("())"));
    }
