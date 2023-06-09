use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name    = "luscomp")]
#[clap(version = "0.1.0")]
#[clap(author  = "Mycroft92 <madhukar DOT yerraguntla AT gmail.com>")]
#[clap(about   = "Yet Another Lustre Compiler")]
//Will change the following later
pub struct CMDArgs {
    #[clap(long, short, default_value_t = String::from("https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"))]
    pub url: String,

    // #[clap(long, short)]
    // pub fetch: bool,

    // #[clap(long, short)]
    // pub gen:  bool,

    // #[clap(long)]
    // pub fetch_n_gen: bool,

    #[clap(long, short, default_value_t = String::from("inst.rs"))]
    pub rust_out: String,

    #[clap(long, short, default_value_t = String::from("instruction_list.yaml"))]
    pub out: String,

    #[clap(long, short, default_value_t = String::from("output.log"))]
    pub log: String,
    
    #[clap(long, short)]
    pub debug: bool
}