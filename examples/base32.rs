/*
    Example os use for radix::{Radix32, Radix32Hex, ZBase32}
    ßÿ Mg // 2016 // License : WTFPL

    Command syntax :
        ./radix32 <d/e> <std/hex/zim> <string>

*/

extern crate radix;


fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("./radix32 <d/e> <std/hex/zim> <string>");
        std::process::exit(0)
    } else if args.len() < 4 {
        println!("Not enough arguments, see base58 --help for help");
        std::process::exit(0)
    }

    let s = args.pop().unwrap();
    let mode = args.pop().unwrap();
    let action = args.pop().unwrap();

    println!("{}", match &*action {
        "e" => {
            match &*mode {
                "std" => radix::Base32::new().encode(s),
                "hex" => radix::Base32Hex::new().encode(s),
                "zim" => radix::ZBase32::new().encode(s),
                x => format!("unknown encoding mode {}", x),
            }
        },
        "q" => {
            match &*mode {
                "std" => radix::Base32::new().encode(s),
                "hex" => radix::Base32Hex::new().encode(s),
                "zim" => radix::ZBase32::new().encode(s),
                x => format!("unknown decoding mode {}", x),
            }
        },
         x  => format!("unknown action {}", x),
    })

}