    #[test]
    fn area_is_width_times_height() {
        assert_eq!(Rectangle { width: 3, height: 4 }.area(), 12);
        assert_eq!(Rectangle { width: 0, height: 9 }.area(), 0);
    }

    #[test]
    fn big_holds_small() {
        let big = Rectangle { width: 30, height: 50 };
        let small = Rectangle { width: 10, height: 40 };
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn same_size_does_not_fit() {
        let a = Rectangle { width: 10, height: 10 };
        let b = Rectangle { width: 10, height: 10 };
        assert!(!a.can_hold(&b));
    }

    #[test]
    fn too_tall_does_not_fit() {
        let wide = Rectangle { width: 50, height: 5 };
        let tall = Rectangle { width: 5, height: 50 };
        assert!(!wide.can_hold(&tall));
    }
