/*
    Charsets for the Radix Rust Library
    ßÿ Mg // 2016 // License : WTFPL

*/

fn get_charset(code: &str) -> std::result::Result<&str, String> {
    match code {
        "b64" => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"),
        "b32" => Ok("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"),
        "zb32" => Ok("ybcdrfg8ejkmcpqxot1uwisza245h769"),
        "b32hex" => Ok("0123456789ABCDEFGHIJKLMNOPQRSTUV"),
        "b16" => Ok("0123456789ABCDEF"),
        "b8" => Ok("01234567"),
        "b85" => Ok("0123456789ABCDEFGHIJKLMNOPQRZTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~"),
        "b58bc" => Ok("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
        _ => Err(format!("unknown charset {}", code)),
    }
}
