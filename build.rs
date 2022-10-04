use std::env;
use std::fs;
use std::path::Path;
use which::which;
use std::process::Command;

fn main() {
    let hello = String::from("Hello, world!");

    let cowsay = which("cowsay");
    let hello_str = if let Ok(path) = cowsay {
        println!("cargo:rustc-cfg=cowsay");
        let stdout = Command::new(path)
            .arg(hello)
            .output()
            .expect("cowsay failed")
            .stdout;
        String::from_utf8_lossy(&stdout).to_string()
    } else {
        hello
    };

    let code = format!("pub mod generated {{
    pub fn say_hello() {{
        println!(r#\"{}\"#)
    }}
}}", hello_str);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(&dest_path, code).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
