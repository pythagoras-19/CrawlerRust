use reqwest;
use scraper::{Html, Selector};

pub(crate) async fn main(link: String) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(&link).await?.text().await?;
    let fragment = Html::parse_document(&body);

    // create a selector and extract the data
    let selector = Selector::parse("p").unwrap();
    for element in fragment.select(&selector) {
        let text = element.text().collect::<Vec<_>>();
        println!("{:?}", text);
    }
    Ok(())
}