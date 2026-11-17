    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn areas() {
        assert!(close(Shape::Circle { radius: 1.0 }.area(), PI));
        assert!(close(Shape::Rectangle { width: 2.0, height: 3.0 }.area(), 6.0));
        assert!(close(Shape::Triangle { base: 4.0, height: 3.0 }.area(), 6.0));
    }

    #[test]
    fn names() {
        assert_eq!(Shape::Circle { radius: 1.0 }.name(), "circle");
        assert_eq!(Shape::Rectangle { width: 1.0, height: 1.0 }.name(), "rectangle");
        assert_eq!(Shape::Triangle { base: 1.0, height: 1.0 }.name(), "triangle");
    }

    #[test]
    fn total() {
        let shapes = vec![
            Shape::Rectangle { width: 2.0, height: 3.0 },
            Shape::Triangle { base: 4.0, height: 3.0 },
        ];
        assert!(close(total_area(&shapes), 12.0));
        assert!(close(total_area(&[]), 0.0));
    }
