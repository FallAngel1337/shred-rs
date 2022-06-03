use structopt::StructOpt;

fn main() {
    let opt = shred_rs::ShredAgrs::from_args();
    shred_rs::shred(opt).unwrap_or_else(|err| {
        eprintln!("Could not shred the file(s) due {err}");
        std::process::exit(1);
    });
}
