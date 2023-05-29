use mini_redis::{client, Result};
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address
    let mut client: client::Client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result: Option<Bytes> = client.get("hello").await?;

    println!("got value from server; result={:?}", result);

    Ok(())
}
