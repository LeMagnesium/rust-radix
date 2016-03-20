extern crate radix;

#[cfg(test)]
mod radix16 {
    #[test]
    fn encode() {
        use radix::Base16;

        let m = Base16::new();
        assert_eq!(m.encode("moo".to_string()), String::from("6D6F6F"));
    }

    #[test]
    fn decode() {
        use radix::Base16;

        let m = Base16::new();
        assert_eq!(m.decode("6D6F6F".to_string()), String::from("moo"));
    }
}
