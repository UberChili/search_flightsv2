use access_token::get_access_token;

mod access_token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let access_token = get_access_token().await?;
    println!("Access token: {access_token}");

    Ok(())
}
