// Time to define the AST for the expression

use pest::iterators::Pair;
use crate::lustre::parser::Rule;
use pest_consume::Error;

#[derive(Debug)]
pub enum DType {
    UINT8,
    UINT16,
    UINT32,
    UINT64,
    INT8,
    INT16,
    INT32,
    INT64,
    FLOAT32,
    FLOAT64,
    BOOL
}

#[derive(Debug)]
pub struct Loc{
    pub start: usize,
    pub end  : usize,
    //Probably I need something else to do this
    // pub fname : String
}

#[derive(Debug)]
pub struct Ident {
    pub  id: String,
    pub pos: Loc
}

//Cool way to just do try
impl<'i> TryFrom<Pair<'i, Rule>> for DType {
    type Error = Error<Rule>;
    fn try_from(pair: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut iter = pair.into_inner().next();
        let val = iter.unwrap().as_str();
        println!("Type: {}", val);
        
        Ok(DType::UINT64)
    }
}


#[derive(Debug)]
pub enum Binop {
    GE ,
    LE ,
    EQ ,
    NE ,
    LSH,
    RSH,
    GT ,
    LT ,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    LAND,
    OR ,
    LOR,
    XOR,
    LXOR,
}


#[derive(Debug)]
pub enum Unop {
    NEG,
    NOT
}

#[derive(Debug)]
pub enum Clock {
    CBase,
    COn(Box<Clock>, Ident, bool)
}


pub type NClock = (Clock, Option<Ident>);
pub type Ann    = (DType, NClock);
pub type LAnn   = (Vec<DType>, NClock);




#[derive(Debug)]
pub enum Const {
    //Can add more as required
    Bool(bool),
    Int (i64),
    Float (f64)
}


#[derive(Debug)]
pub enum Exp {
    Econst(Const),
    Etup(Vec<Exp>), // Probably I can redo what Lustre did
    Evar(Ident, Option<Ann>),
    Eunop(Unop, Box<Exp>, Option<Ann>),
    Ebinop(Binop, Box<Exp>, Box<Exp>, Option<Ann>),
    Efby(Box<Exp>, Box<Exp>, Option<LAnn>),
    Earrow(Box<Exp>, Box<Exp>, Option<LAnn>),
    Ewhen(Box<Exp>, Ident, bool, Option<LAnn>),
    Emerge(Ident, Box<Exp>, Box<Exp>, Option<LAnn>),
    Eite(Box<Exp>, Box<Exp>, Box<Exp>, Option<LAnn>),
    Eapp(Ident, Vec<Exp>, Option<Box<Exp>>, Option<LAnn>)
}

#[derive(Debug)]
pub enum PureExp {
    Econst(Const),
    Evar(Ident, Option<Ann>),
    Eunop(Unop, Box<Exp>, Option<Ann>),
    Ebinop(Binop, Box<Exp>, Box<Exp>, Option<Ann>),
    Efby(Box<Exp>, Box<Exp>, Option<LAnn>),
    Earrow(Box<Exp>, Box<Exp>, Option<LAnn>),
    Ewhen(Box<Exp>, Ident, bool, Option<LAnn>),
    Emerge(Ident, Box<Exp>, Box<Exp>, Option<LAnn>),
    Eite(Box<Exp>, Box<Exp>, Box<Exp>, Option<LAnn>),
    Eapp(Ident, Vec<Exp>, Option<Box<Exp>>, Option<LAnn>)
}

#[derive(Debug)]
pub struct Node {

}