
use pest::Parser;
use crate::errors;
use pest_consume::{Error};

// // include the grammar file so that Cargo knows to rebuild this file on grammar changes
// const _GRAMMAR: &str = include_str!("lustre/syntax.pest");

#[derive(Parser)]
#[grammar="lustre/syntax.pest"]
pub struct LustreParser;

type ParseRes<T> = Result<T, Error<Rule>>;

impl LustreParser {
    
    pub fn parse_string() {
            let parsed_res = LustreParser::parse(Rule::constant, "0x27");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::constant, "true");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::file, "027L");
            println!("{:?}", parsed_res);
    }
}

#[test]
fn test1() {
    LustreParser::parse_string();
}