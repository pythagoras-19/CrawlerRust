use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send http get request
    let body = reqwest::get("https://www.rust-lang.org/").await?.text().await?;
    let fragment = Html::parse_document(&body);

    // create a selector and extract the data
    let selector = Selector::parse("p").unwrap();
    for element in fragment.select(&selector) {
        let text = element.text().collect::<Vec<_>>();
        println!("{:?}", text);
    }
    Ok(())
}