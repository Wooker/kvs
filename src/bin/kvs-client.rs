use std::net::{Ipv4Addr, SocketAddrV4};

use clap::{Command, SubCommand, Arg};
use kvs::{client::{KvsClient, ClientError, ClientResult}, Command as KvsCommand};

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

fn main() -> ClientResult<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let socket = SocketAddrV4::new(localhost, 8080);

    let mut client = KvsClient::new(socket).expect("Could not connect to socket");

    let m = cli().get_matches();
    match m.subcommand() {
        Some(("set", sub_matches)) => {
            let command = KvsCommand::Set(
                sub_matches.get_one::<String>("KEY").unwrap().to_string(),
                sub_matches.get_one::<String>("VALUE").unwrap().to_string(),
            );

            client.send_command(command).expect("Could not send command");
            Ok(())
        }
        Some(("get", sub_matches)) => {
            let command = KvsCommand::Get(
                sub_matches.get_one::<String>("KEY").unwrap().to_string(),
            );

            client.send_command(command).expect("Could not send command");
            Ok(())
        }
        Some(("rm", sub_matches)) => {
            let command = KvsCommand::Rm(
                sub_matches.get_one::<String>("KEY").unwrap().to_string(),
            );

            client.send_command(command).expect("Could not send command");
            Ok(())
        }
        _ => { return Err(ClientError::NoArgs); }
    }
}
