
use pest::{ Parser, pratt_parser::{Assoc, Op, PrattParser}};
use crate::errors;
use pest_consume::{match_nodes, Error};

// // include the grammar file so that Cargo knows to rebuild this file on grammar changes
// const _GRAMMAR: &str = include_str!("lustre/syntax.pest");


// Define the precedence of binary operations. We use `lazy_static` so that
// this is only ever constructed once.
lazy_static::lazy_static! {
    static ref PRATT: PrattParser<Rule> =
    PrattParser::default();
}


#[derive(Parser)]
#[grammar="lustre/syntax.pest"]
pub struct LustreParser;

type ParseRes<T> = Result<T, Error<Rule>>;

// pub fn parse_constant(pairs: Pairs<Rule>) -> {

// } 

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