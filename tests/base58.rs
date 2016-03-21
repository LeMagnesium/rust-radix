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
    fn encoderipple() {
        use radix::Base58Ripple;

        let m = Base58Ripple::new();
        assert_eq!(m.encode("moo".to_string()), String::from("N4QPX==="));
    }

    #[test]
    fn encodeflickr() {
        use radix::Base58Flickr;

        let m = Base58Flickr::new();
        assert_eq!(m.encode("moo".to_string()), String::from("enpow==="));
    }

    #[test]
    fn decodebc() {
        use radix::Base58BC;

        let m = Base58BC::new();
        assert_eq!(m.decode("ENQPX===".to_string()), String::from("moo"));
    }

    #[test]
    fn decoderipple() {
        use radix::Base58Ripple;

        let m = Base58Ripple::new();
        assert_eq!(m.decode("N4QPX===".to_string()), String::from("moo"));
    }

    #[test]
    fn decodeflickr() {
        use radix::Base58Flickr;

        let m = Base58Flickr::new();
        assert_eq!(m.decode("enpow===".to_string()), String::from("moo"));
    }
}
