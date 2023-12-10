use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("unable to remove file");
    println!("file is removed");
}
