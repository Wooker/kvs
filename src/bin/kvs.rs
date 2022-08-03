use clap::{App, Command, SubCommand, Arg};

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

fn main() {
    let mut kvs = kvs::KvStore::new();
    kvs.set("a".to_string(), "b".to_string());
    let m = cli().get_matches();

    match m.subcommand() {
        Some(("set", sub_matches)) => {
            kvs.set(sub_matches.get_one::<String>("KEY").unwrap().to_string(), sub_matches.get_one::<String>("VALUE").unwrap().to_string());
        }
        Some(("get", sub_matches)) => {
            let val = kvs.get(sub_matches.get_one::<String>("KEY").unwrap().to_string()).unwrap();
            println!("{}", val);
        }
        _ => {}
    }
}
