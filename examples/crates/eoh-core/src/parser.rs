use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "eoh.pest"]
pub struct EohParser;

pub fn parse_file(unparsed_file: &str) {
    let parse_result = EohParser::parse(Rule::program, unparsed_file);

    match parse_result {
        Ok(mut pairs) => {
            println!("✅ Succes! Fișierul respectă gramatica Eye of Horus.");
            for pair in pairs.next().unwrap().into_inner() {
                match pair.as_rule() {
                    Rule::decl_vertex => println!("Am găsit un VERTEX în spațiu!"),
                    Rule::decl_tetra => println!("Am găsit un TETRAHEDRON!"),
                    Rule::decl_pulse => println!("ATENȚIE: Declanșare Puls Higgs!"),
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Eroare de geometrie/sintaxă în cod:\n{}", e);
        }
    }
}