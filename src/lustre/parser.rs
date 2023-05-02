
use pest::Parser;
use pest::iterators::Pairs;
use pest::pratt_parser::{PrattParser, Assoc::*, Op};
use crate::errors;
use crate::lustre::ast;
use pest_consume::{match_nodes, Error};

// // include the grammar file so that Cargo knows to rebuild this file on grammar changes
// const _GRAMMAR: &str = include_str!("lustre/syntax.pest");



#[derive(Parser)]
#[grammar="lustre/syntax.pest"]
pub struct LustreParser;

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

    

    pub fn parse_string() {
            let parsed_res = LustreParser::parse(Rule::constant, "0x27");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::constant, "true");
            println!("{:?}", parsed_res);

            let parsed_res = LustreParser::parse(Rule::constant, "027L");
            println!("{:?}", parsed_res);
    }
}

#[test]
fn test1() {
    LustreParser::parse_string();
}