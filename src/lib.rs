include!("./charsets.rs");

pub struct Base85;
pub struct Base64;
pub struct Base32;
pub struct ZBase32;
pub struct Base32Hex;
pub struct Base16;
pub struct Base8;

fn encode(s: String, basename: &str, radix: usize) -> Result<String, String> {
    let mut result: String = String::new();
    let mut binpattern: String = String::new();
    let lcm = format!("{:b}", radix).len() - 1;

    let charset = try!(get_charset(basename));

    for b in s.as_bytes() {
        let mut pattern = format!("{:b}", b);
        while pattern.len() < 8 {pattern = format!("0{}", pattern)}

        binpattern.push_str(&*pattern);

        while binpattern.len() > lcm {
            let slc = usize::from_str_radix(&binpattern[0..lcm], 2).unwrap();

            binpattern = binpattern[lcm..].to_string();
            result.push_str(&charset[slc..slc+1]);
        }
    }

    // Complete if some bytes are left over
    if binpattern.len() > 0 {
        while binpattern.len() % lcm > 0 {binpattern.push_str("00000000");}

        while binpattern.len() > 0 {
            let slc = usize::from_str_radix(&binpattern[0..lcm], 2).unwrap();
            binpattern = binpattern[lcm..].to_string();
            result.push_str(if slc == 0 {"="} else {&charset[slc..slc+1]});
        }
    }

    Ok(result)
}

fn decode(s: String, basename: &str, radix: usize) -> Result<String, String> {
    let mut result: String = String::new();
    let mut binpattern: String = String::new();
    let lcm = format!("{:b}", radix).len() - 1;

    let charset = try!(get_charset(basename));

    for b in s.chars() {
        if b == '=' {
            continue
        } else {
            let idx = charset.find(b);
            let mut k = format!("{:b}", idx.unwrap());
            while k.len() < lcm {k = format!("0{}", k);}

            binpattern.push_str(&*k);
        }

        while binpattern.len() >= 8 {
            let it = vec![u8::from_str_radix(&binpattern[0..8], 2).unwrap()];
            binpattern = binpattern[8..].to_string();

            let chr = std::string::String::from_utf8(it);
            if chr.is_err() {panic!("Invalid UTF8 string");}

            result.push_str(chr.unwrap().as_str());
        }
    }

    Ok(result)
}

impl Base85 {
    pub fn new() -> Base85 { Base85{} }

    pub fn encode(&self, s: String) -> String {
        encode(s, "b85", 85).unwrap()
    }

    pub fn decode(&self, s: String) -> String {
        decode(s, "b85", 85).unwrap()
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
