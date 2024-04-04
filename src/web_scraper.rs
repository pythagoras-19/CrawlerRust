use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send http get request
    let response = reqwest::get("https://www.rust-lang.org/").await?.text().await?;
    println!("Hello!!");
    println!("response: {}", response);

    Ok(())
}