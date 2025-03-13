use std::str::FromStr;

mod file;
fn main() {
    println!("Hello, world!");

    dbg!(crate::file::File::new(
        std::path::PathBuf::from_str(std::env::args().nth(1).unwrap().as_str()).unwrap(),
    ));
}
