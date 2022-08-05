use std::{net::{Ipv4Addr, SocketAddrV4}, env::current_dir};

use kvs::server::{KvsServer, ServerResult};

fn main() -> ServerResult<()> {
    let localhost = Ipv4Addr::LOCALHOST;
    let socket = SocketAddrV4::new(localhost, 8080);

    let mut server = KvsServer::new(socket, current_dir().unwrap()).expect("Could not start the server");

    server.handle()?;

    Ok(())
}
