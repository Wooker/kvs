use std::net::{Ipv4Addr, SocketAddrV4};

use clap::{Arg, Command, SubCommand};
use kvs::client::{ClientError, ClientResult, KvsClient};

fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a value to a key")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("A string value")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get a value with key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a value with key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
}

fn main() -> ClientResult<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let plover = Ipv4Addr::new(90, 156, 230, 97);
    let socket = SocketAddrV4::new(localhost, 8080);

    let mut client = KvsClient::new(socket)?;

    let m = cli().get_matches();
    match m.subcommand() {
        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap().to_string();
            let val = sub_matches.get_one::<String>("VALUE").unwrap().to_string();

            client.set(key, val)?;
            Ok(())
        }
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap().to_string();

            let key = client.get(key)?;
            println!("{}", key);
            Ok(())
        }
        Some(("rm", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap().to_string();

            client.rm(key)?;
            Ok(())
        }
        None => {
            cli().print_help()?;
            Ok(())
        }
        _ => {
            return Err(ClientError::WrongArgs);
        }
    }
}
