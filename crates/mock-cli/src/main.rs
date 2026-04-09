use std::io::{self, Read};

fn print_help() {
    eprintln!("Usage: mock [OPTIONS] [INPUT]");
    eprintln!();
    eprintln!("Mock text with alternating case");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  [INPUT]  Input text to mock (reads from stdin if omitted)");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -g, --garble  Enable garble (phonetic replacement) mode");
    eprintln!("  -h, --help    Print help");
}

fn main() {
    let mut garble = false;
    let mut input = None;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-g" | "--garble" => garble = true,
            s if s.starts_with('-') => {
                eprintln!("error: unexpected argument '{s}'");
                eprintln!();
                print_help();
                std::process::exit(2);
            }
            _ => {
                if input.is_some() {
                    eprintln!("error: unexpected argument '{arg}'");
                    eprintln!();
                    print_help();
                    std::process::exit(2);
                }
                input = Some(arg);
            }
        }
    }

    let input = match input {
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
    let output = mock_core::build_mocking_text(input, garble);
    println!("{output}");
}
