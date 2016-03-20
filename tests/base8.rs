extern crate radix;

#[cfg(test)]

mod radix8 {
    #[test]
    fn encode() {
        use radix::Base8;

        let m = Base8::new();
        assert_eq!(m.encode("moo".to_string()), String::from("33267557"));
    }

    #[test]
    fn decode() {
        use radix::Base8;

        let m = Base8::new();
        assert_eq!(m.decode("33267557".to_string()), String::from("moo"));
    }
}
