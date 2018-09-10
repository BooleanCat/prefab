#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "prefab")]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
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
        Subcommand::Create {name} => create(name),
        Subcommand::State {name} => state(name),
        Subcommand::Delete {name} => delete(name),
    }
    std::process::exit(1);
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
