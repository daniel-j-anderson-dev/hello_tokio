use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get IP address from command line arguments
    let mini_redis_ip = std::env::args().nth(1)
        .unwrap_or(String::from("127.0.0.1:6379")); // localhost:{default mini-redis port}

    // Connect to the mini-redis instance
    let mut client = client::connect(&mini_redis_ip).await?;

    // Set desired key and vlaue
    let (key, value) = ("Hello", "World".into());

    // Set a key value pair in the DB
    client.set(key, value).await?;

    // Get the value for a corresponding key
    let response = client.get(key).await?;

    println!("The server@{mini_redis_ip} responded to the get request with\n{response:#?}");

    return Ok(());
}