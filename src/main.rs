#[macro_use]
extern crate structopt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "prefab")]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(name = "create", raw(global_settings = "&[AppSettings::DisableVersion]"))]
    Create {
        #[structopt(name = "NAME")]
        name: String,
    },

    #[structopt(name = "state", raw(global_settings = "&[AppSettings::DisableVersion]"))]
    State {
        #[structopt(name = "NAME")]
        name: String,
    },

    #[structopt(name = "delete", raw(global_settings = "&[AppSettings::DisableVersion]"))]
    Delete {
        #[structopt(name = "NAME")]
        name: String,
    },

}

fn main() {
    match Opt::from_args().command {
        Subcommand::Create {name} => create(name),
        Subcommand::State {name} => state(name),
        Subcommand::Delete {name} => delete(name),
    }
}

fn create(_: String) {}

fn state(_: String) {}

fn delete(_: String) {}
