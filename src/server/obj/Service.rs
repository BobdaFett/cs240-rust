use std::net::{TcpListener, TcpStream};

pub struct Service {
    // Needs to store anything needed for the connection.
    // Is essentially the Bank object, so I;m going to try to structure it like that as well.
    listener: TcpListener,
    stream: TcpStream,

}

impl Service {
    pub fn new() {
        
    }
}