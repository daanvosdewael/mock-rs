use std::io::{self, Read};

use clap::Parser;

#[derive(Parser)]
#[command(name = "mock", about = "Mock text with alternating case")]
struct Cli {
    /// Enable garble (phonetic replacement) mode
    #[arg(short, long)]
    garble: bool,

    /// Input text to mock (reads from stdin if omitted)
    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = match cli.input {
        Some(text) => text,
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("failed to read stdin");
            buf
        }
    };
    let input = input.trim_end_matches('\n');
    if input.is_empty() {
        eprintln!("error: no input provided");
        std::process::exit(1);
    }
    let output = mock_core::build_mocking_text(input, cli.garble);
    println!("{output}");
}
