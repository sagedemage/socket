//! Client Program

use tokio::{net::{TcpStream}};
use std::env;
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() -> io::Result<()> {
    // The message as an array of 8 bit unsinged integers
    let mut buf = String::new();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            eprintln!("Missing input!");
            process::exit(0);
        }
        3 => {
            let option: &String = &args[1];
            let input: &String = &args[2];

            if option == "i" {
                if input == "" {
                    eprintln!("Input is empty!");
                }
                else {
                    let msg: &[u8] = input.as_bytes();
                    send_message_to_server(msg).await?;
                }
            }
            else {
                eprintln!("Option does not exist!");
            }
        }
        _ => {
            // Prompt user input
            print!("Enter message: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;

            // remove newline from input
            buf = buf.replace('\n', "");

            if buf.is_empty() {
                println!("Input is empty!");
            } else {
                let msg: &[u8] = buf.as_bytes();
                send_message_to_server(msg).await?;
            }
        }
    }
    Ok(())
}

async fn send_message_to_server(message: &[u8]) -> io::Result<()> {
    //! Sends a message to the server
    // Open a connection to the server's address
    let client: TcpStream = TcpStream::connect("127.0.0.1:6379").await?;

    // Send a message to the server
    client.try_write(message)?;
    println!("Sent a message to the server.");
    
    Ok(())
}
