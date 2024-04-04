use reqwest;
use scraper::{Html, Selector};

pub(crate) struct WebScraper {
    link: String,
    selectors: Vec<String>,
}

impl WebScraper {
    pub fn new(link: String, selectors: Vec<String>) -> Self {
        WebScraper {
            link,
            selectors,
        }
    }

    pub async fn scrape(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);

        // create a selector and extract the data
        for selector_str in &self.selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in fragment.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                println!("{:?}", text);
            }
        }
        Ok(())
    }
}