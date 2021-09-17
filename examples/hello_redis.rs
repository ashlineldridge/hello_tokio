use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let w = say_world();

    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Get key "hello"
    let result = client.get("hello").await;
    println!("Got result 1 {:?}", result);

    // Set the key "hello" with value "world"
    let result = client.set("hello", "ashlin".into()).await;

    println!("Got result 2 {:?}", result);

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    w.await;

    Ok(())
}

async fn say_world() {
    println!("world");
}
