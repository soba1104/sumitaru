use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::io::{Read, Write};

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

    pub fn run(&self) -> Result<(), Error> {
        let listener = try!(TcpListener::bind(self.addr));
        let (mut socket, _addr) = try!(listener.accept());
        echo(&mut socket)
    }
}

fn echo(socket: &mut TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 10];
    try!(socket.read(&mut buffer));
    try!(socket.write(&buffer));
    Ok(())
}
