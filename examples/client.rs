//use mini_redis::{client, Result};
use std::{io::Error};
use tokio::{net::{TcpStream}};

#[tokio::main]
async fn main() {
    // Open a connection to the mini-redis address
    //let mut client: client::Client = client::connect("127.0.0.1:6379").await?;

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
    /*
    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result: Option<Bytes> = client.get("hello").await?;

    println!("got value from server; result={:?}", result);
    */
}
