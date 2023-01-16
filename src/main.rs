use std::fs;

fn main() {
    let paths = fs::read_dir("../resources/frames").unwrap();
    for path in paths {
        println!("Filename: {}", path.unwrap().path().display());
    }
}
