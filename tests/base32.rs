extern crate radix;

#[cfg(test)]
mod radix32 {
    #[test]
    fn encode() {
        use radix::Base32;

        let m = Base32::new();
        assert_eq!(m.encode("moo".to_string()), String::from("NVXW6==="));
    }

    #[test]
    fn zencode() {
        use radix::ZBase32;

        let m = ZBase32::new();
        assert_eq!(m.encode("moo".to_string()), String::from("pizs6==="))
    }

    #[test]
    fn encodehex() {
        use radix::Base32Hex;

        let m = Base32Hex::new();
        assert_eq!(m.encode("moo".to_string()), String::from("DLNMU==="))
    }

    #[test]
    fn decode() {
        use radix::Base32;

        let m = Base32::new();
        assert_eq!(m.decode("NVXW6===".to_string()), String::from("moo"));
    }

    #[test]
    fn zdecode() {
        use radix::ZBase32;

        let m = ZBase32::new();
        assert_eq!(m.decode("pizs6===".to_string()), String::from("moo"));
    }

    #[test]
    fn decodehex() {
        use radix::Base32Hex;

        let m = Base32Hex::new();
        assert_eq!(m.decode("DLNMU===".to_string()), String::from("moo"))
    }
}
