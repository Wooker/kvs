use std::{net::{Ipv4Addr, SocketAddrV4}, env::current_dir};

use kvs::{server::{KvsServer, ServerResult}, thread_pool::{ThreadPool, shared::SharedQueueThreadPool}, engines::kvstore::KvStore};

fn main() -> ServerResult<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let socket = SocketAddrV4::new(localhost, 8080);

    //let pool = NaiveThreadPool::new(4)?;
    let pool = SharedQueueThreadPool::new(4).unwrap();
    let engine = KvStore::open(current_dir()?)?;

    let mut server = KvsServer::new(engine, pool)?;

    server.run(socket)?;

    println!("Server stoped running");
    Ok(())
}
