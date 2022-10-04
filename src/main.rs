include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    if cfg!(cowsay) {
        println!("Beware of cows!")
    }
    generated::say_hello();
}
