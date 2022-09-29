use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("New connection!");
        handle_stream(&mut stream);
    }
}

/// This function receives a new connection to the
/// server. Then it reads what a client wrote and
/// prints it.
fn handle_stream(connection: &mut TcpStream) {
    let mut buffer = [0u8; 4];
    match connection.read_exact(&mut buffer) {
        Ok(_) => {
            println!("{:?}", u32::from_be_bytes(buffer));
        }
        Err(err) => {
            println!("An error occured: {}", err);
        }
    };
}
