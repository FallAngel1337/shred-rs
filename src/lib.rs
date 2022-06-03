use structopt::StructOpt;
use std::rc::Rc;

mod shred;

pub use shred::shred;

type ShredConf = Rc<ShredAgrs>;
type IoResult<T> = std::io::Result<T>;

#[derive(Debug, StructOpt, Clone)]
pub struct ShredAgrs {
    #[structopt(short = "n", default_value = "3",
    help = "Number of rounds")]
    pub rounds: usize,
    
    #[structopt(short = "u", help = "Delete after shred it")]
    pub delete: bool,

    #[structopt(short = "s", default_value = "4096",
    help = "Amount of bytes to shred")]
    pub bytes: usize,
    
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(short, long)]
    pub force: bool,

    #[structopt(short = "z", help = "Fill with zeros to hide shred-rs")]
    pub zero: bool,

    #[structopt(name = "FILES")]
    pub files: Vec<String>,
}

