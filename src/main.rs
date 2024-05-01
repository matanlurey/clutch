use std::io::{Error, Write};

use clutch::Lexer;

#[allow(clippy::missing_errors_doc)]
pub fn main() -> Result<(), Error> {
    println!("Welcome to the Clutch REPL!");
    println!("TIP: Press Ctrl-C to exit.\n");

    // A miniature REPL that reads lines from the user and prints them back.
    // On SIGINT (Ctrl-C), the program will exit.
    loop {
        let mut input = String::new();

        // Prompt with a > character.
        print!("> ");

        // Flush the output to ensure the prompt is displayed.
        std::io::stdout().flush()?;

        match std::io::stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                // Print every token in the input.
                let lexer = Lexer::new(&input);
                for token in lexer {
                    println!("  {token:?}");
                }
            }
            Err(e) => return Err(e),
        }

        // Newline after the input.
        println!();
    }

    Ok(())
}
