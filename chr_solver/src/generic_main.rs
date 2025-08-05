use pest::Parser;

mod chr_parser;

fn main() {
    let input = "path('London','Berlin',1100)";
    match chr_parser::CHRParser::parse(chr_parser::Rule::constraint, input) {
        Ok(pairs) => {
            println!("Parsed constraint: {:?}", pairs);
        }
        Err(e) => {
            println!("Error parsing constraint: {}", e);
        }
    }
}
