//! Server Program

use std::io;
use tokio::net::{TcpListener, TcpStream};
use socket_cli::green_message;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Notify the user that the server is running
    // Using 8-16 Hexadecimal Colors
    let msg1: String = green_message("Server is running!");
    let msg2: String = green_message("------------------");
    println!("{msg1}");
    println!("{msg2}");

    // Bind the listener to the address
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // The second item contains the IP and port of the new connect
        let (socket, _): (TcpStream, std::net::SocketAddr) = listener.accept().await?;

        // A new task is spawned for each inbound socket.
        // The socket is moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    //! Print the message when the client sends a message to the server
    // The buffer as an array of 8 bit unsinged integers

    // Wait for the socket to be readable
    match socket.readable().await {
        Ok(_) => {
            // create buffer as an array of 8 bit unsinged integers
            let mut buf: [u8; 4096] = [0; 4096];

            // Try to read the data from the stream
            match socket.try_read(&mut buf) {
                Ok(0) => {
                    eprintln!("Buffer is empty!");
                }
                Ok(_size) => {
                    // Concert the buffer to a string
                    match std::str::from_utf8(&buf) {
                        Ok(msg) => {
                            println!("{msg}");
                        }
                        Err(err) => {
                            eprintln!("{err}");
                        }
                    };
                }
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}
