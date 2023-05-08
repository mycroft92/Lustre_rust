
use pest::Parser;
use pest::iterators::{Pair,Pairs};
use pest::pratt_parser::{PrattParser, Assoc::*, Op};
use crate::errors::{self, LusRes};
use crate::lustre::ast;
use pest_consume::{match_nodes, Error};
use std::path::Path;

// // include the grammar file so that Cargo knows to rebuild this file on grammar changes
// const _GRAMMAR: &str = include_str!("lustre/syntax.pest");



#[derive(Parser)]
#[grammar="lustre/syntax.pest"]
pub struct LustreParser{
    fname: String,
}

type ParseRes<T> = Result<T, Error<Rule>>;

// pub fn parse_constant(pairs: Pairs<Rule>) -> {

// } 
lazy_static::lazy_static! {
    static ref PRATT: PrattParser<Rule> =
    PrattParser::new()
        .op(Op::infix(Rule::COMMA, Left))
        .op(Op::infix(Rule::ARR, Right))
        .op(Op::infix(Rule::FBY, Right))
        .op(Op::infix(Rule::OR, Right) | Op::infix(Rule::LOR, Right) | Op::infix(Rule::AND, Right) | Op::infix(Rule::LAND, Right) | Op::infix(Rule::EQ, Right) 
            | Op::infix(Rule::NE, Right)| Op::infix(Rule::GT, Right) | Op::infix(Rule::LT, Right) | Op::infix(Rule::GE, Right) | Op::infix(Rule::LE, Right) 
            | Op::infix(Rule::LSH, Left) | Op::infix(Rule::RSH, Left))
        .op(Op::infix(Rule::WHEN, Left)  | Op::infix(Rule::WHENOT, Left))
        .op(Op::infix(Rule::ADD, Left) | Op::infix(Rule::SUB, Left))
        .op(Op::infix(Rule::MUL, Left)| Op::infix(Rule::DIV, Left) | Op::infix(Rule::MOD, Left));
}




impl LustreParser {


    // fn parse_expression(pairs: Pairs<Rule>) -> ParseRes<ast::Exp> {
    //     PRATT_PARSER
    //     .map_primary(|primary| match primary.as_rule() {
    //         Rule::mrg_exp1 => Expr::Integer(primary.as_str().parse::<>().unwrap()),
    //         Rule::expr => parse_expr(primary.into_inner()),
    //         rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
    //     })
    //     .map_infix(|lhs, op, rhs| {
    //         let op = match op.as_rule() {
    //             Rule::add => Op::Add,
    //             Rule::subtract => Op::Subtract,
    //             Rule::multiply => Op::Multiply,
    //             Rule::divide => Op::Divide,
    //             Rule::modulo => Op::Modulo,
    //             rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
    //         };
    //         Expr::BinOp {
    //             lhs: Box::new(lhs),
    //             op,
    //             rhs: Box::new(rhs),
    //         }
    //     })
    //     .map_prefix(|op, rhs| match op.as_rule() {
    //         Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
    //         _ => unreachable!(),
    //     })
    //     .parse(pairs)
    // }

    pub fn parse_file (path: &Path) -> LusRes<Vec<ast::Node>> {
        let content = std::fs::read_to_string(path).map_err(|err| {
            crate::errors::Error{
                kind: crate::errors::ErrorKind::FileNotFoundError, 
                pos: None, 
                msg: format!("Failed to read {}: {err}", path.to_string_lossy())}
        })?;

        let parsed_res = LustreParser::parse(Rule::file,&content);
        //Fix this one
        Ok(vec![])
    }

    pub fn parse_string() {
            let parsed_res = LustreParser::parse(Rule::constant, "0x27");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::constant, "true");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::mrg_exp1, "merge a (true -> x) (false -> y)").expect("Unexpected parse");
            println!("{:?}", parsed_res);
            for pair in parsed_res {
                match pair.as_rule() {
                    Rule::mrg_exp1 => {LustreParser::parse_mrg1(pair); ()},
                    _              => println!("Got here")
                };
            }
            let pairs = LustreParser::parse(Rule::type_name, "uint32").expect("Unknown type!");
            for pair in pairs {
                let parsed_res = LustreParser::match_type(pair);
                println!("{:?}", parsed_res);
            }

            // println!("{:?}", parsed_res);
    }
}

// fn match_type(ty: str) -> ParseRes<ast::DType> {
//     if (ty == "uint8")
// }
impl LustreParser {
    fn match_type(pair: Pair<Rule>) -> ast::DType {
        match pair.as_str() {
            "uint8"   => ast::DType::UINT8,
            "uint16"  => ast::DType::UINT16,
            "uint32"  => ast::DType::UINT32,
            "uint64"  => ast::DType::UINT64,
            "int8"    => ast::DType::INT8,
            "int16"   => ast::DType::INT16,
            "int32"   => ast::DType::INT32,
            "int64"   => ast::DType::INT64,
            "float32" => ast::DType::FLOAT32,
            "float64" => ast::DType::FLOAT64,
            "bool"    => ast::DType::BOOL,
            _         => unreachable!("Panic while parsing an unimplemented type")

        }

    }

    fn parse_expression(pair: Pair<Rule>) -> ParseRes<Vec<ast::Exp>> {

    }

    // fn parse_primary(pair: Pair<Rule>) -> ParseRes<ast::Exp> {
    //     match pair.as_rule() {
    //         Rule::mrg_exp1 => LustreParser::parse_mrg1(pair)
    //     }

    // }
        //write a function for each type like this and call parse_expression finally
    fn parse_id(pair: Pair<Rule>) -> ParseRes<ast::Ident>{
        let loc = pair.as_span();
        let ident = ast::Ident {id: String::from(loc.as_str())};
        Ok(ident)
    }

    fn parse_mrg(pair: Pair<Rule>) -> ParseRes<ast::Exp> {
        let mut items = pair.into_inner();
        println!("Rules: {:?}",items);
        let token= items.next().unwrap();
        //The following two must be present
        let ident = LustreParser::parse_id(token)?;
        let texp = LustreParser::parse_expression(items.next().unwrap())?;
        let fexp = LustreParser::parse_expression(items.next().unwrap())?;
        Ok(ast::Exp::Emerge(ident, texp, fexp, None))
    }

    fn parse_mrg1(pair: Pair<Rule>) -> ParseRes<ast::Exp> {
        let mut items = pair.into_inner();
        //unwrap should work always since the rule succeeded
        println!("Rules: {:?}",items);
        let token= items.next().unwrap();
        
        let ident = LustreParser::parse_id(token)?;
        println!("Check: {:?}", ident);
        // let mut t_branch = vec![];
        // match 

        Ok(ast::Exp::Emerge(ident, vec![], vec![], None))
    }
}

#[test]
fn test1() {
    LustreParser::parse_string();
}