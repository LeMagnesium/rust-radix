extern crate radix;

fn main() {
    use radix::Base8;

    let m = Base8::new();
    m.encode("moo".to_string());
}
