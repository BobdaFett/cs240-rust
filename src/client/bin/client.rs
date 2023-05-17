use std::{
    net::TcpStream,
    io::{prelude::*, stdin, stdout}
};
use colored::Colorize;

fn main() -> std::io::Result<()> {
    // Get input from the user.
    let mut input = String::new();
    print!("Enter the IP address of the server (x.x.x.x:<port>): ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    print!("Attempting to connect to {}... ", input.trim());

    // Attempt to connect to the server.
    if let Ok(stream) = TcpStream::connect(input.trim()) {
        println!("{}", "Connected!".green());
        handle(&stream)?;
    } else {
        println!("{}", "Connection failed.".red());
    }

    Ok(())
}
 
fn handle(mut stream: &TcpStream) -> std::io::Result<()> {
    stream.write("Hello there!".as_bytes())?;
    stream.flush().unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    println!("{:?}", response);

    Ok(())
}
