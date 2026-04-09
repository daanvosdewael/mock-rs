use clap::Parser;

#[derive(Parser)]
#[command(name = "mock", about = "Mock text with alternating case")]
struct Cli {
    /// Enable garble (phonetic replacement) mode
    #[arg(short, long)]
    garble: bool,

    /// Input text to mock
    input: String,
}

fn main() {
    let cli = Cli::parse();
    let output = mock_core::build_mocking_text(&cli.input, cli.garble);
    println!("{output}");
}
