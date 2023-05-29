use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

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
    // The `Connection` lets use read/write redis **frames** instead of
    // bytes streams. The `Connection` type is defined by mini-redis.
    let mut connection: Connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response: Frame = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
