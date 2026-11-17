    #[test]
    fn integers() {
        assert_eq!(largest(&[34, 50, 25, 100, 65]), 100);
        assert_eq!(largest(&[-5, -2, -9]), -2);
    }

    #[test]
    fn characters() {
        assert_eq!(largest(&['y', 'm', 'a', 'q']), 'y');
    }

    #[test]
    fn floats() {
        assert_eq!(largest(&[1.5, 0.2, 9.75]), 9.75);
    }

    #[test]
    fn single_item() {
        assert_eq!(largest(&[7]), 7);
    }
