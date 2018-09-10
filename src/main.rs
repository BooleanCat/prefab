#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "V", long = "version")]
    version: bool,

    #[structopt(subcommand)]
    command: Option<Subcommand>,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(name = "create")]
    Create {
        #[structopt(name = "NAME")]
        name: String,
    },

    #[structopt(name = "state")]
    State {
        #[structopt(name = "NAME")]
        name: String,
    },

    #[structopt(name = "delete")]
    Delete {
        #[structopt(name = "NAME")]
        name: String,
    },

}

fn main() {
    let opt = Opt::from_args();

    match opt.command {
        None => base(opt),
        Some(Subcommand::Create {name}) => create(name),
        Some(Subcommand::State {name}) => state(name),
        Some(Subcommand::Delete {name}) => delete(name),
    }
    std::process::exit(1);
}

fn base(opt: Opt) {
    if opt.version {
        println!("dev");
        std::process::exit(0);
    }
}

fn create(_: String) {
    std::process::exit(0);
}

fn state(_: String) {
    std::process::exit(0);
}

fn delete(_: String) {
    std::process::exit(0);
}
