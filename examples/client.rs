use std::{io::Error};
use tokio::{net::{TcpStream}};

#[tokio::main]
async fn main() {
    // Open a connection to the address
    let client: Result<TcpStream, Error> = TcpStream::connect("127.0.0.1:6379").await;

    let msg = b"Hello";

    match client {
        Ok(stream) => {
            println!("Connected to the server!");
            let write_stream = stream.try_write(msg);

            match write_stream {
                Ok(_size) => {}
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
