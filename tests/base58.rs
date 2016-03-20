extern crate radix;

#[cfg(test)]

mod radix58 {
    #[test]
    fn encodebc() {
        use radix::Base58BC;

        let m = Base58BC::new();
        assert_eq!(m.encode("moo".to_string()), String::from("ENQPX==="));
    }

    #[test]
    fn decodebc() {
        use radix::Base58BC;

        let m = Base58BC::new();
        assert_eq!(m.decode("ENQPX===".to_string()), String::from("moo"));
    }
}
