use structopt::StructOpt;
use std::fmt::{self,Display,Formatter};

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct AppArgs {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// add operation
    #[structopt(name = "add")]
    Add(Elements),

    /// times operation
    #[structopt(name = "times")]
    Times(Elements),
}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<u32>,
}

impl Display for Elements {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{:#?}]", self.elements)
    }
}

fn main() {
    let opt = AppArgs::from_args();

    match opt.command { 
        Command::Add(e) => {
            let result = e.elements.iter().fold(0, |aac, &x|aac + x);
            println!("Operants: {}, result:{}",e,result);
        },
        Command::Times(e) => {
            let result = e.elements.iter().fold(1, |aac, &x|aac * x);
            println!("Operants: {}, result:{}",e,result);
        },
    }
}