use std::io::Write;

fn main() {
    let string = String::from("pub struct Test;");
    let mut x = std::fs::OpenOptions::new().create(true).write(true).open("src/out.rs").unwrap();
    x.write_all(string.as_bytes()).unwrap();

    let string = String::from("pub mod out;\npub use out::*;");
    let mut x = std::fs::OpenOptions::new().append(true).open("src/lib.rs").unwrap();
    x.write_all(string.as_bytes()).unwrap();
}