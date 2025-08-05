use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "chr.pest"]
pub struct CHRParser;

pub fn parse_rules(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    CHRParser::parse(Rule::rules, input)
}
