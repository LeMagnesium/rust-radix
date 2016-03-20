extern crate radix;

#[cfg(test)]

mod radix85 {
    #[test]
    fn encode() {
        use radix::Base85;

        let m = Base85::new();
        assert_eq!(m.encode("moo".to_string()), String::from("RMzl"));
    }

    #[test]
    fn decode() {
        use radix::Base85;

        let m = Base85::new();
        assert_eq!(m.decode("RMzl".to_string()), String::from("moo"));
    }
}
