use std::net::ToSocketAddrs;

use tokio::net::TcpStream;

pub struct KeyzConnection {
    pub host: String,
    pub port: u16
}

impl KeyzConnection {
    pub fn new(host: String, port: u16) -> KeyzConnection {
        KeyzConnection {
            host: host,
            port: port
        }
    }

    pub async fn connect(&self) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.host, self.port)
            .to_socket_addrs()?
            .next()
            .ok_or("[-] Couldn't resolve to address")?;

        TcpStream::connect(addr).await.map_err(Into::into)
    }
}