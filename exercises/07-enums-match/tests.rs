    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn circle() {
        assert!(close(Shape::Circle(1.0).area(), PI));
        assert!(close(Shape::Circle(2.0).area(), 4.0 * PI));
    }

    #[test]
    fn square() {
        assert!(close(Shape::Square(3.0).area(), 9.0));
    }

    #[test]
    fn rectangle() {
        assert!(close(Shape::Rectangle(2.0, 3.5).area(), 7.0));
    }
