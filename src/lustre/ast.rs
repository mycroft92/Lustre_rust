// Time to define the AST for the expression

use pest::iterators::Pair;

#[derive(Debug)]
pub enum DType {
    UINT64,
    INT64,
    UFLOAT64,
    FLOAT64,
    BOOL
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
    COn(Box<Clock>, String, bool)
}


pub type NClock = (Clock, Option<String>);
pub type Ann    = (DType, NClock);
pub type LAnn   = (Vec<DType>, NClock);




#[derive(Debug)]
pub enum Const {
    //Can add more as required
    Int (i64),
    Float (f64)
}

#[derive(Debug)]
pub enum Exp {
    Econst(Const),
    Evar(String, Ann),
    Eunop(Unop, Box<Exp>, Ann),
    Ebinop(Binop, Box<Exp>, Box<Exp>, Ann),
    Efby(Vec<Exp>, Vec<Exp>, LAnn),
    Earrow(Vec<Exp>, Vec<Exp>, LAnn),
    Ewhen(Vec<Exp>, String, bool, LAnn),
    Emerge(String, Vec<Exp>, Vec<Exp>, LAnn),
    Eite(Box<Exp>, Vec<Exp>, Vec<Exp>, LAnn),
    Eapp(String, Vec<Exp>, Option<Box<Exp>>, LAnn)
}
