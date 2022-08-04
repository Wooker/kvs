use std::env::current_dir;

use clap::{Command, SubCommand, Arg};
use kvs::{KvStore, Result, KvsError};

fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("set")
            .about("Set a value to a key")
            .arg(Arg::with_name("KEY").help("A string key").required(true))
            .arg(Arg::with_name("VALUE").help("A string value").required(true))
        )
        .subcommand(SubCommand::with_name("get")
            .about("Get a value with key")
            .arg(Arg::with_name("KEY").help("A string key").required(true))
        )
        .subcommand(SubCommand::with_name("rm")
            .about("Remove a value with key")
            .arg(Arg::with_name("KEY").help("A string key").required(true))
        )
}

fn main() -> Result<()>{
    let m = cli().get_matches();

    match m.subcommand() {
        Some(("set", sub_matches)) => {
            let mut kvs = KvStore::open(current_dir()?)?;

            kvs.set(
                sub_matches.get_one::<String>("KEY").unwrap().to_string(),
                sub_matches.get_one::<String>("VALUE").unwrap().to_string()
            )?;
        }
        Some(("get", sub_matches)) => {
            let kvs = KvStore::open(current_dir()?)?;

            let val = kvs.get(sub_matches.get_one::<String>("KEY").unwrap().to_string())?;
            println!("{}", val.unwrap());
        }
        Some(("rm", sub_matches)) => {
            let mut kvs = KvStore::open(current_dir()?)?;

            kvs.remove(sub_matches.get_one::<String>("KEY").unwrap().to_string())?;
        }
        _ => { return Err(KvsError::NoArgs); }
    }

    Ok(())
}
