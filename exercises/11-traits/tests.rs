    #[test]
    fn dog_says_woof() {
        let dog = Dog { name: "Rex".to_string() };
        assert_eq!(dog.speak(), "Rex says Woof");
    }

    #[test]
    fn cat_says_meow() {
        let cat = Cat { name: "Tom".to_string() };
        assert_eq!(cat.speak(), "Tom says Meow");
    }

    #[test]
    fn loud_version_is_uppercase() {
        let dog = Dog { name: "Rex".to_string() };
        assert_eq!(dog.speak_loudly(), "REX SAYS WOOF");
    }

    #[test]
    fn works_through_dyn_trait() {
        let animal: Box<dyn Speak> = Box::new(Cat { name: "Kit".to_string() });
        assert_eq!(animal.speak(), "Kit says Meow");
    }
