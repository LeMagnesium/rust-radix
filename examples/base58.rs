/*
    Example os use for radix::{Radix58BC, Radix58Ripple, Radix58Flickr}
    ßÿ Mg // 2016 // License : WTFPL

    Command syntax :
        ./radix85 <d/e> <bc/ripple/flickr> <string>

*/

extern crate radix;


fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("./radix58 <d/e> <bc/ripple/flickr> <string>");
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
                "bc" => radix::Base58BC::new().encode(s),
                "ripple" => radix::Base58Ripple::new().encode(s),
                "flickr" => radix::Base58Flickr::new().encode(s),
                x => format!("unknown encoding mode {}", x),
            }
        },
        "q" => {
            match &*mode {
                "bc" => radix::Base58BC::new().encode(s),
                "ripple" => radix::Base58Ripple::new().encode(s),
                "flickr" => radix::Base58Flickr::new().encode(s),
                x => format!("unknown decoding mode {}", x),
            }
        },
         x  => format!("unknown action {}", x),
    })

}
