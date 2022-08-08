use std::{net::{Ipv4Addr, SocketAddrV4}, env::current_dir};

use kvs::{server::{KvsServer, ServerResult}, thread_pool::{ThreadPool, NaiveThreadPool}, engines::kvstore::KvStore};

fn main() -> ServerResult<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let socket = SocketAddrV4::new(localhost, 8080);

    let pool = NaiveThreadPool::new(4)?;
    let engine = KvStore::open(current_dir()?)?;

    let mut server = KvsServer::new(engine, pool)?;

    server.run(socket)?;

    Ok(())
}
