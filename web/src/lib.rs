use smol::{block_on, net};
use std::error::Error;

#[cold]
pub fn main() -> Result<(), Box<dyn Error>> {
    let tcp = net::TcpListener::bind("127.0.0.1:9090");

    block_on(async move {
        let tcp = tcp.await.expect("TCP binding error.");
    });

    Ok(())
}
