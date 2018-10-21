mod config;

#[macro_use]
extern crate structopt;

extern crate serde;

#[cfg_attr(test, macro_use)]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use config::Config;

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

        #[structopt(name = "PATH-TO-BUNDLE", parse(from_os_str))]
        bundle_path: PathBuf,
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
        Subcommand::Create {name, bundle_path} => create(name, bundle_path),
        Subcommand::State {name} => state(name),
        Subcommand::Delete {name} => delete(name),
    }
}

fn create(_: String, _: PathBuf) {
    let _: Config = Default::default();
}

fn state(_: String) {}

fn delete(_: String) {}
