use pac2;

use pac2::Pacfile;

use std::io::Read;

fn main() {
    let mut pacfile = Pacfile::from_file("data.p2i").unwrap();
    let contents = pacfile.read("src/main.rs").unwrap();
    let mut buffer = String::new();
    (&contents[..]).read_to_string(&mut buffer).unwrap();
    println!("Contents: {:?}", buffer);
}
