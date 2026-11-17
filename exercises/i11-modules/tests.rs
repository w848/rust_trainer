    #[test]
    fn conversions() {
        assert_eq!(temperature::c_to_f(100.0), 212.0);
        assert_eq!(temperature::c_to_f(-40.0), -40.0);
        assert_eq!(temperature::f_to_c(212.0), 100.0);
    }

    #[test]
    fn feelings() {
        assert_eq!(temperature::describe::feeling(5.0), "cold");
        assert_eq!(temperature::describe::feeling(10.0), "nice");
        assert_eq!(temperature::describe::feeling(25.0), "hot");
    }

    #[test]
    fn feelings_in_fahrenheit() {
        assert_eq!(temperature::describe::feeling_f(32.0), "cold");
        assert_eq!(temperature::describe::feeling_f(50.0), "nice");
        assert_eq!(temperature::describe::feeling_f(100.0), "hot");
    }
