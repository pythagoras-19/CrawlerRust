use reqwest;
use scraper::{Html, Selector};

pub(crate) struct WebScraper {
    link: String,
}

impl WebScraper {
    pub fn new(link: String) -> Self {
        WebScraper {
            link,
        }
    }

    pub async fn call(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);

        // create a selector and extract the data
        let selector = Selector::parse("p").unwrap();
        for element in fragment.select(&selector) {
            let text = element.text().collect::<Vec<_>>();
            println!("{:?}", text);
        }
        Ok(())
    }
}