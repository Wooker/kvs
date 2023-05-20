use std::{
    env::current_dir,
    net::{Ipv4Addr, SocketAddrV4},
};

use clap::{Parser, Subcommand};
use kvs::{
    engines::kvstore::KvStore,
    server::{KvsServer, ServerResult},
    thread_pool::{shared::SharedQueueThreadPool, ThreadPool},
};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct ServerCli {
    #[arg(short, long)]
    address: Option<Ipv4Addr>,
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> ServerResult<()> {
    let cli = ServerCli::parse();

    let address = cli.address.unwrap_or(Ipv4Addr::LOCALHOST);
    let port = cli.port.unwrap_or(8000);
    let socket = SocketAddrV4::new(address, port);

    //let pool = NaiveThreadPool::new(4)?;
    let pool = SharedQueueThreadPool::new(4).unwrap();
    let engine = KvStore::open(current_dir()?)?;

    let mut server = KvsServer::new(engine, pool)?;

    server.run(socket)?;

    println!("Server stoped running");
    Ok(())
}
