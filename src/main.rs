#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate quick_error;

mod error;
mod server;
use server::Server;
use error::Error;

use std::env;
use std::process;

fn fatal(e: &Error) -> ! {
    error!("fatal: {}", e);
    process::exit(1);
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let host = &args[1];
    let port = args[2].parse::<u16>().unwrap();

    let server = Server::new(host, port).unwrap_or_else(|e| fatal(&e));
    println!("Hello, world! {:?}", server);
}
