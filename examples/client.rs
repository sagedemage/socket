//! Client Program

use tokio::{net::{TcpStream}};
use std::env;
use std::io::{self, Write};
use std::process;
use socket_cli::{blue_message, red_message};

#[tokio::main]
async fn main() -> io::Result<()> {
    // The message as an array of 8 bit unsinged integers
    let mut buf = String::new();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // Prompt user input
            let msg = blue_message("Enter a message: ");
            print!("{msg}");
            io::stdout().flush()?;
            io::stdin().read_line(&mut buf)?;

            // remove newline from input
            buf = buf.replace('\n', "");

            if buf.is_empty() {
                let msg: String = red_message("Input is empty!");
                eprintln!("{msg}");
            } else {
                let msg: &[u8] = buf.as_bytes();
                send_message_to_server(msg).await?;
            }
        }
        2 => {
            let option: &String = &args[1];
            if option == "i" {
                let msg: String = red_message("Missing input!");
                eprintln!("{msg}");
                process::exit(0);
            }
            else {
                let msg: String = red_message("Option does not exist!");
                eprintln!("{msg}");
                process::exit(0);
            }
        }
        3 => {
            let option: &String = &args[1];
            let input: &String = &args[2];

            if option == "i" {
                if input == "" {
                    let msg: String = red_message("Input is empty!");
                    eprintln!("{msg}");
                }
                else {
                    let msg: &[u8] = input.as_bytes();
                    send_message_to_server(msg).await?;
                }
            }
            else {
                let msg: String = red_message("Option does not exist!");
                eprintln!("{msg}");
            }
        }
        _ => {
            
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
    println!("\x1b[32m{}\x1b[0m", "Sent a message to the server!");
    
    Ok(())
}
