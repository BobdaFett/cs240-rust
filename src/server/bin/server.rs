use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*}, thread
};
use colored::Colorize;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2345").expect("Failed to bind.");

    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed.");
        let handle = thread::spawn(move || {
            handle(&stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
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
