use tokio::{net::{TcpStream}};
use std::env;
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() -> io::Result<()> {
    // The message as an array of 8 bit unsinged integers
    let mut buf = String::from("");

    let args: Vec<String> = env::args().collect();

    let mut option: &String = &String::from("");
    let mut input: &String = &String::from("");

    if args.len() == 2 {
        eprintln!("Missing input!");
        process::exit(0);
    }

    else if args.len() == 3 {
        option = &args[1];
        input = &args[2];
    }

    if option == "i" {
        let msg: &[u8] = input.as_bytes();
        send_message_to_server(msg).await?;
    }

    else if option == "" {
        // Prompt user input
        print!("Enter message: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buf)?;

        // remove newline from input
        buf = buf.replace("\n", "");

        match buf.len() {
            0 => {
                println!("Input is empty!")
            }
            _ => {
                let msg: &[u8] = buf.as_bytes();
                send_message_to_server(msg).await?;
            }
        }
    }

    Ok(())
}

async fn send_message_to_server(message: &[u8]) -> io::Result<()> {
    /* Sends a message to the server */
    // Open a connection to the server's address
    let client: TcpStream = TcpStream::connect("127.0.0.1:6379").await?;

    // Send a message to the server
    client.try_write(message)?;
    println!("Sent a message to the server.");
    
    Ok(())
}
