use std::env;
use std::fs;
use std::path::Path;
use indoc::indoc;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(
        &dest_path, indoc! {"
        pub mod generated {
            pub fn some_function() {
                println!(\"Hello from some_function!\")
            }
        }"}
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
