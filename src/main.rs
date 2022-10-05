include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(cowsay)]
fn print_warning() {
    println!("Beware of cows!")
}

#[cfg(not(cowsay))]
fn print_warning() {}

fn main() {
    print_warning();
    generated::say_hello();
}
