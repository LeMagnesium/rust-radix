/*
    Rust Radix Manipulation Library
    ßý Mg // 2016 // License: WTFPL

    Available combinations of charsets/radix :
        b8      / 8     -> Base8;
        b16     / 16    -> Base16;
        b32     / 32    -> Base32;
        zb32    / 32    -> ZBase32;
        b32hex  / 32    -> Base32Hex;
        b64     / 64    -> Base64;
        b58bc   / 58    -> Base58BC;
        b58ripe / 58    -> Base58Ripe;  (NIY)
        b58flkr / 58    -> Base58Flickr (NIY)

*/

include!("./charsets.rs");

pub struct Base64;
pub struct Base58BC;
pub struct Base32;
pub struct ZBase32;
pub struct Base32Hex;
pub struct Base16;
pub struct Base8;

pub fn encode(s: String, basename: &str, radix: usize) -> Result<String, String> {
    assert!(radix > 1);
    assert!(basename.to_string().len() > 0);

    let mut success: bool = true;
    let mut result: String = String::new();
    let mut binpattern: String = String::new();

    let lcm = format!("{:b}", radix).len() - 1;
    let charset = match get_charset(basename) {
        Ok(x) => x,
        Err(_) => "",
    };

    'encodeloop: for b in s.as_bytes() {
        binpattern.push_str(&*format!(
                "{:8b}", b
            ).replace(" ", "0")
        );

        result.push_str(
                if binpattern.len() > lcm {
                    match usize::from_str_radix(&*(binpattern.drain(0..lcm).collect::<String>()), 2) {
                        Ok(x) => &charset[x..x+1],
                        Err(_) => {success = false; break 'encodeloop;},
                    }
                } else {
                    ""
                }
        )
    };

    // Complete if some bytes are left over
    if binpattern.len() > 0 {
        while binpattern.len() % lcm > 0 {binpattern.push_str("00000000");}

        while binpattern.len() > 0 {
            result.push_str(
                match usize::from_str_radix(&*(binpattern.drain(0..lcm).collect::<String>()), 2) {
                    Ok(x) => if x == 0 {"="} else {&charset[x..x+1]},
                    Err(_) => {success = false; break;}
                }
            );
        };
    };

    match success {
        true => Ok(result),
        false => Err("error in decoding string".to_string()),
    }
}

pub fn decode(s: String, basename: &str, radix: usize) -> Result<String, String> {
    assert!(radix > 1);
    assert!(basename.to_string().len() > 0);

    let mut success: bool = true;
    let mut result: String = String::new();
    let mut binpattern: String = String::new();

    let lcm = format!("{:b}", radix).len() - 1;
    let charset = match get_charset(basename) {
        Ok(x) => x,
        Err(_) => "",
    };

    'decodeloop: for b in s.chars() {
        if b == '=' {
            continue;
        }

        binpattern.push_str(&*(
            match charset.find(b) {
                None => String::new(),
                Some(x) => format!("{:8b}", x).replace(" ", "0")[8-lcm..8].to_string(),
            }
        ));

        result.push_str(&*(
            if binpattern.len() >= 8 {
                match u8::from_str_radix(&*(binpattern.drain(0..8).collect::<String>()), 2) {
                    Ok(x) => match std::string::String::from_utf8(vec![x]) {
                        Ok(y) => y,
                        Err(_) => {success = false; break 'decodeloop}
                    },
                    Err(_) => {success = false; break 'decodeloop},
                }
            } else {
                String::new()
            }
        ))
    }

    match success {
        true => Ok(result),
        false => Err("error in decoding string".to_string()),
    }
}

impl Base64 {
    pub fn new() -> Base64 { Base64{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b64", 64).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b64", 64).unwrap()
    }
}

impl Base58BC {
    pub fn new() -> Base58BC { Base58BC{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b58bc", 58).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b58bc", 58).unwrap()
    }
}

impl Base32 {
    pub fn new() -> Base32 { Base32{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b32", 32).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b32", 32).unwrap()
    }
}

impl ZBase32 {
    pub fn new() -> ZBase32 { ZBase32{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "zb32", 32).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "zb32", 32).unwrap()
    }
}

impl Base32Hex {
    pub fn new() -> Base32Hex { Base32Hex{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b32hex", 32).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b32hex", 32).unwrap()
    }
}

impl Base16 {
    pub fn new() -> Base16 { Base16{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b16", 16).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b16", 16).unwrap()
    }
}

impl Base8 {
    pub fn new() -> Base8 { Base8{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b8", 8).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b8", 8).unwrap()
    }
}
