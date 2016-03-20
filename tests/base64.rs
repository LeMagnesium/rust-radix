extern crate radix;

#[cfg(test)]
mod radix64 {
    #[test]
    fn encode() {
        use radix::Base64;

        let m = Base64::new();
        assert_eq!(m.encode("moo".to_string()), String::from("bW9v"));
        assert_eq!(m.encode("any carnal pleasure.".to_string()), String::from("YW55IGNhcm5hbCBwbGVhc3VyZS4="));
        assert_eq!(m.encode("sure.".to_string()), String::from("c3VyZS4="));
        assert_eq!(
            m.encode("Man is distinguished, not only by his reason, but by this singular passion from \
                other animals, which is a lust of the mind, that by a perseverance of delight \
                in the continued and indefatigable generation of knowledge, exceeds the short \
                vehemence of any carnal pleasure.".to_string()),
            String::from("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
                IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
                dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
                dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
                ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=")
        );
    }

    #[test]
    fn decode() {
        use radix::Base64;

        let m = Base64::new();
        assert_eq!(m.decode("bW9v".to_string()), String::from("moo"));
        assert_eq!(m.decode("YW55IGNhcm5hbCBwbGVhc3VyZS4=".to_string()), String::from("any carnal pleasure."));
        assert_eq!(m.decode("c3VyZS4=".to_string()), String::from("sure."));
        assert_eq!(
            m.decode("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
                IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
                dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
                dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
                ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=".to_string()),
            String::from("Man is distinguished, not only by his reason, but by this singular passion from \
                other animals, which is a lust of the mind, that by a perseverance of delight \
                in the continued and indefatigable generation of knowledge, exceeds the short \
                vehemence of any carnal pleasure.")
        );
    }
}

fn main() {
    use radix::Base64;

    let m = Base64::new();
    println!("{}", m.decode("bW9v".to_string()));
    let k = m.encode("Man is distinguished, not only by his reason, but by this singular passion from \
        other animals, which is a lust of the mind, that by a perseverance of delight \
        in the continued and indefatigable generation of knowledge, exceeds the short \
        vehemence of any carnal pleasure.".to_string());
    let u = String::from("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlz\
        IHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2Yg\
        dGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGlu\
        dWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRo\
        ZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=");
    println!("{}", k);
    println!("{}", u);
    println!("{:?}", u == k);
}
