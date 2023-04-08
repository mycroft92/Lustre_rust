
use pest::Parser;
#[derive(Parser)]
#[grammar="syntax.pest"]
pub struct LustreParser;