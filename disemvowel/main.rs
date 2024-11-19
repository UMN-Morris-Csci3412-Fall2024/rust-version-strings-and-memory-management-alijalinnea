pub mod disemvowel;

use crate::disemvowel::disemvowel;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Not enough arguments");
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let input_content = read_file(input_path);
    let disemvoweled_content = disemvowel(&input_content);
    write_file(output_path, &disemvoweled_content);
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}

fn write_file(path: &Path, content: &str) {
    fs::write(path, content).expect("Unable to write file");
}
