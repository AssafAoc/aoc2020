use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::{Path, PathBuf};

mod d1;
mod d2;

fn get_input(day: u8, suffix: &str) -> impl Iterator<Item=String> {
    let file_path = {
        let mut tmp = PathBuf::new();
        tmp.push("resources");
        tmp.push(format!("d{}{}.txt", day, suffix));
        tmp
    };
    let file = File::open(&file_path).expect(&format!("file: {:?} not found", file_path));
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.unwrap()).filter(|s| s.len() > 0)
}

fn main() {
    // d1::run();
    d2::run();
}
