use std::io::{self, Write};

use ferrum_core::command::parse;

fn main() {
    println!("ferrum {}", ferrum_core::VERSION);

    let mut line = String::new();
    loop {
        print!("ferrum> ");
        io::stdout().flush().unwrap();

        line.clear();

        let n = io::stdin().read_line(&mut line).unwrap();
        if n == 0 || line.trim() == "exit" {
            break;
        }
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            continue;
        }

        let cmd = parse(trimmed_line);
        match cmd {
            Some(cmd) => println!("{:?}", cmd),
            None => println!("Unknown command: {}", line),
        }
    }
}
