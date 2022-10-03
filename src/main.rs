include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    println!("Hello, world!");
    generated::some_function();
}
