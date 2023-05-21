use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::{
    fs::{self, File},
    io::{BufRead, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
fn main() -> Result<()> {
    let listener = TcpListener::bind(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000))?;
    // listener
    //     .set_nonblocking(true)
    //     .expect("Cannot set non-blocking");

    let peers_path = Path::new("peers.txt");
    let contents = fs::read_to_string(peers_path)?;

    let peers: Vec<SocketAddr> = contents
        .lines()
        .map(|addr| addr.parse::<SocketAddr>().unwrap())
        .collect();

    thread::scope(|s| {
        s.spawn(move || loop {
            for peer in peers.iter() {
                let mut stream = TcpStream::connect(peer).unwrap();
                writeln!(stream, "Hello").unwrap();
            }

            thread::sleep(Duration::from_secs(1));
        });

        s.spawn(move || {
            for incoming in listener.incoming() {
                let stream = incoming.unwrap();
                let mut reader = BufReader::new(&stream);
                let mut buf = String::new();
                reader.read_line(&mut buf).unwrap();
                dbg!(&buf);
            }
        });
    });

    Ok(())
}
