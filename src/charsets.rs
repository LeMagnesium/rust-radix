/*
    Charsets for the Radix Rust Library
    ßÿ Mg // 2016 // License : WTFPL

*/

fn get_charset(code: &str) -> std::result::Result<&str, String> {
    match code {
        // https://en.wikipedia.org/wiki/Base64
        "b64" => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"),
        // https://en.wikipedia.org/wiki/Base32
        "b32" => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"),
        // https://en.wikipedia.org/wiki/Base32#z-base-32
        "zb32" => Ok("ybcdrfg8ejkmcpqxot1uwisza245h769"),
        // https://en.wikipedia.org/wiki/Base32#base32hex
        "b32hex" => Ok("0123456789ABCDEFGHIJKLMNOPQRSTUV"),
        // https://en.wikipedia.org/wiki/Base16
        "b16" => Ok("0123456789ABCDEF"),
        "b8" => Ok("01234567"),
        // https://en.wikipedia.org/wiki/Ascii85
        //"b85" => Ok("0123456789ABCDEFGHIJKLMNOPQRZTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~"),
        // https://en.wikipedia.org/wiki/Base58
        "b58bc" => Ok("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
        "b58rpl" => Ok("rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz"),
        "b58flkr" => Ok("123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"),
	"b94" => Ok("!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ'[]^_`abcdefghijklmnopqrstuvwxyz{|}~"),
        _ => Err(format!("unknown charset {}", code)),
    }
}
