use std::{future::Future, net::TcpStream};

struct PollingStream<'a> {
    stream: &'a TcpStream,
}

impl<'a> Future for PollingStream<'a> {
    type Output = u8;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

fn main() {}
