use std::io::{self, Write};
use meval::eval_str;

fn main() {
    println!("Trigonometry Calculator");
    println!("Type 'exit' to quit");

    loop {
        print!("Enter expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match eval_str(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}
