use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::{
    fs::{self, File},
    io::{BufRead, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
struct Cli {
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(long)]
    peers_file: Option<PathBuf>, //Vec<SocketAddrV4>>,
}

use anyhow::Result;
use clap::Parser;
use kvs::command;
fn main() -> Result<()> {
    let cli = Cli::parse();
    dbg!(&cli);

    let port = cli.port.or(Some(8080)).unwrap();
    let listener = TcpListener::bind(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port))?;
    // listener
    //     .set_nonblocking(true)
    //     .expect("Cannot set non-blocking");

    let peers_path = Path::new("peers.txt");
    let contents = fs::read_to_string(peers_path)?;

    let peers: Vec<SocketAddr> = contents
        .lines()
        .map(|addr| addr.parse::<SocketAddr>().expect("Invalid socket address"))
        // .map(|addr| TcpStream::connect(addr.parse::<SocketAddr>().unwrap()).unwrap())
        .collect();
    dbg!(&peers);

    thread::scope(|s| {
        s.spawn(move || loop {
            for peer in peers.iter() {
                if peer.port() == port {
                    continue;
                }
                if let Ok(mut stream) = TcpStream::connect(peer) {
                    writeln!(stream, "Hello").unwrap();
                }

                // writeln!(peer, "Hello").unwrap();
                println!("Wrote");
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
