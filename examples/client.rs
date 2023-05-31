use std::{io::Error};
use tokio::{net::{TcpStream}};
use std::{env, io};
use std::process;

#[tokio::main]
async fn main() {
    // The message as an array of 8 bit unsinged integers
    let mut msg: &[u8] = b"Hello";
    let mut buf = String::new();

    let args: Vec<String> = env::args().collect();

    let mut option: &String = &String::from("");
    let mut input: &String = &String::from("");

    if args.len() == 3 {
        option = &args[1];
        input = &args[2];
    }

    else if args.len() == 2 {
        eprintln!("Missing input!");
        process::exit(0);
    }

    if option == "i" {
        if input != "" {
            msg = input.as_bytes();
        }
    }

    else if option == "" {
        println!("Enter message: ");
        let stdin = io::stdin();
        let result = stdin.read_line(&mut buf);

        // remove newline from input
        buf = buf.replace("\n", "");

        match result {
            Ok(_size) => {
                msg = buf.as_bytes();
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    send_message_to_server(msg).await; 
}

async fn send_message_to_server(message: &[u8]) {
    /* Sends a message to the server */
    // Open a connection to the server's address
    let client: Result<TcpStream, Error> = TcpStream::connect("127.0.0.1:6379").await;
    
    // Send a message to the server
    match client {
        Ok(stream) => {
            let write_stream = stream.try_write(message);
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
