    #[test]
    fn plain_numbers() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn fizz_for_multiples_of_3() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn buzz_for_multiples_of_5() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn fizzbuzz_for_multiples_of_15() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(45), "FizzBuzz");
    }
