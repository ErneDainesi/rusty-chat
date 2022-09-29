use std::io::{Read, Result, stdin, BufReader, BufRead, Write};
use std::net::TcpStream;

fn main() {
    let address = "127.0.0.1:8080";
    println!("Connecting to {}...", address);
    match handle_client(address, &mut stdin()) {
        Ok(_) => {
            println!("Connection finished correctly!");
        }
        Err(err) => {
            println!("Failed while connecting to server: {}", err);
        }
    }
    
}

fn handle_client(address: &str, stream: &mut dyn Read) -> Result<()> {
    let stdin_reader = BufReader::new(stream);
    let mut socket = TcpStream::connect(address)?;

    for input in stdin_reader.lines() {
        let input = input?;
        println!("Sending {}...", input);
        socket.write(input.as_bytes())?;
        socket.write("\n".as_bytes())?;
    }

    Ok(())
}
