/*
    Example os use for radix::Radix8
    ßÿ Mg // 2016 // License : WTFPL

    Command syntax :
        ./radix8 <d/e> <string>

*/

extern crate radix;

enum Modes {
    Encode,
    Decode,
    Unknown,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--help" {
        println!("./radix8 <d/e> <string>");
        std::process::exit(0)
    } else if args.len() < 3 {
        println!("Not enough arguments, see base8 --help for help");
        std::process::exit(0)
    }

    let mode = match &*args[1] {
        "e" => Modes::Encode,
        "d" => Modes::Decode,
         _  => Modes::Unknown,
    };

    use radix::Base8;
    let m = Base8::new();

    println!("{}", match mode {
        Modes::Encode => m.encode(args[2].to_string()),
        Modes::Decode => m.decode(args[2].to_string()),
        Modes::Unknown => format!("Unknown mode {}", &*args[1]),
    })
}