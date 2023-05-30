use core::panic;
use std::net::{SocketAddr};
use tokio::{net::{TcpListener, TcpStream}};
use std::str;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connect
        let (socket, _): (TcpStream, SocketAddr) = listener.accept().await.unwrap();

        // A new task is spawned for each inbound socket. 
        // The socket is moved to the new task and processed there. 
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    /* Print the message when the client sends a message to the server */
    // The buffer as an array of 8 bit unsinged integers
    let mut buf: [u8; 4096] = [0; 4096];

    let read_stream = socket.try_read(&mut buf);

    match read_stream {
        Ok(0) => {
            println!("Buffer is empty");
        }
        Ok(_size) => {
            // Concert the buffer to a string
            let msg = match str::from_utf8(&buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };

            println!("{}", msg);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
