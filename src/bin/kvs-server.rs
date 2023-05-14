use std::{
    env::current_dir,
    net::{Ipv4Addr, SocketAddrV4},
};

use kvs::{
    engines::kvstore::KvStore,
    server::{KvsServer, ServerResult},
    thread_pool::{shared::SharedQueueThreadPool, ThreadPool},
};

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
