use std::str::FromStr;

mod file;
fn main() {
    println!("Hello, world!");

    let path = std::path::PathBuf::from_str(std::env::args().nth(1).unwrap().as_str()).unwrap();

    dbg!(crate::file::File::read_directory(path));
    //dbg!(crate::file::File::new(path.clone()));
}
