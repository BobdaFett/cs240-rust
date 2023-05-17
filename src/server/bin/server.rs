use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*}
};
use colored::Colorize;

fn main() -> std::io::Result<()> {
    // We want our server to listen on the local ip, port 2345. Same as the original program.
    let listener = TcpListener::bind("127.0.0.1:2345")?;
    // Alternatively...
    // let listener = TcpListener::bind("127.0.0.1:2345").unwrap();

    for stream in listener.incoming() {
        // handle the stream.
        handle(&stream?)?;
    }

    Ok(())
}

fn handle(mut stream: &TcpStream) -> std::io::Result<()> {
    println!("{}", "Connection established!".green());

    let mut request = String::new();
    stream.read_to_string(&mut request)?;
    println!("{:?}", request);

    stream.write("General Kenobi!".as_bytes())?;
    stream.flush().unwrap();

    Ok(())
}
