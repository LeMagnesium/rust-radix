/*
    Example os use for radix::Radix16
    ßÿ Mg // 2016 // License : WTFPL

    Command syntax :
        ./radix16 <d/e> <string>

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
        println!("./radix16 <d/e> <string>");
        std::process::exit(0)
    } else if args.len() < 3 {
        println!("Not enough arguments, see base16 --help for help");
        std::process::exit(0)
    }

    let mode = match &*args[1] {
        "e" => Modes::Encode,
        "d" => Modes::Decode,
         _  => Modes::Unknown,
    };

    use radix::Base16;
    let m = Base16::new();

    println!("{}", match mode {
        Modes::Encode => m.encode(args[2].to_string()),
        Modes::Decode => m.decode(args[2].to_string()),
        Modes::Unknown => format!("Unknown mode {}", &*args[1]),
    })
}
