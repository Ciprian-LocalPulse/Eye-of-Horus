//! Command-line placeholder for Eye of Horus.

use eoh_core::{implementation_status, parser, phi_pi_address};
use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--status") | None => {
            println!("Eye of Horus - {}", implementation_status());
            println!("Example phi-pi address M(1): {}", phi_pi_address(1));
        }
        // 👇 Am adăugat comanda nouă pentru a rula Lexer-ul
        Some("parse") => {
            if let Some(file_path) = args.next() {
                // Dacă utilizatorul oferă un fișier (ex: `cargo run parse hello_world.eoh`)
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        println!("👁️ Parsing spatial data from: {}", file_path);
                        parser::parse_file(&content);
                    }
                    Err(e) => {
                        eprintln!("❌ Error reading file '{}': {}", file_path, e);
                        std::process::exit(1);
                    }
                }
            } else {
                // Dacă nu oferă un fișier, rulăm codul mock pentru testare rapidă
                println!("👁️ No file provided. Running spatial parser on mock data...");
                let mock_code = "ORIGIN 0.0, 0.0, 0.0\nVERTEX A 1.0, 1.618, 3.141\nPULSE_HIGGS ORIGIN, v=1.0";
                parser::parse_file(mock_code);
            }
        }
        Some("--help") | Some("-h") => {
            println!("Eye of Horus CLI");
            println!("Usage:");
            println!("  eoh --status        Show implementation status");
            println!("  eoh parse <file>    Parse a .eoh file using the spatial lexer");
            println!("  eoh --help          Print this message");
            println!("\nNote: The full spatial runtime is planned, currently in parser testing phase.");
        }
        Some(other) => {
            eprintln!("unknown argument: {other}");
            std::process::exit(2);
        }
    }
}
