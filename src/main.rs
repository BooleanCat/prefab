#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "V", long = "version")]
    version: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.version {
        println!("dev");
        std::process::exit(0);
    }
    std::process::exit(1);
}
