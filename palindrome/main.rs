mod palindrome;

use crate::palindrome::is_palindrome;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin palindrome <string>");
        std::process::exit(1);
    }

    let input = &args[1];
    if is_palindrome(input) {
        println!("'{}' is a palindrome!", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
