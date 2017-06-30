use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use error::Error;

#[derive(Debug)]
pub struct Server {
    host: IpAddr,
    port: u16,
    addr: SocketAddr,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Result<Server, Error> {
        let host = try!(IpAddr::from_str(host));
        let server = Server{
            host: host,
            port: port,
            addr: SocketAddr::new(host, port),
        };
        Ok(server)
    }
}
