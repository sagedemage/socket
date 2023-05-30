use std::{io::Error};
use tokio::{net::{TcpStream}};

#[tokio::main]
async fn main() {
    // Open a connection to the address
    let client: Result<TcpStream, Error> = TcpStream::connect("127.0.0.1:6379").await;

    // The message as an array of 8 bit unsinged integers
    let msg: &[u8] = b"Hello";

    match client {
        Ok(stream) => {
            let write_stream = stream.try_write(msg);
            match write_stream {
                Ok(_size) => {
                    println!("Sent a message to the server.");
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
