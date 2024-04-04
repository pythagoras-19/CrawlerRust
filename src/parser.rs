use reqwest;
use scraper::{Html, ElementRef};
use std::collections::HashSet;

pub async fn generate_selectors(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    let fragment = Html::parse_document(&body);
    let mut selectors = HashSet::new();
    for element in fragment.root_element().descendants() {
        if let Some(element_ref) = ElementRef::wrap(element) {
            let tag_name = element_ref.value().name();
            selectors.insert(tag_name.to_string());
        }
    }
    Ok(selectors.into_iter().collect())
}