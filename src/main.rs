mod web_scraper;
mod parser;

#[tokio::main]
async fn main() {
    use std::io::{stdin,stdout,Write};
    let mut str =String::new();
    print!("Please enter a website: ");
    let _=stdout().flush();
    stdin().read_line(&mut str).expect("Did not enter a correct string");
    if let Some('\n')= str.chars().next_back() {
        str.pop();
    }
    if let Some('\r')= str.chars().next_back() {
        str.pop();
    }
    println!("You typed: {}", str);
    let selectors = parser::generate_selectors(&str).await.unwrap();
    call_web_scraper(str, selectors).await;
}

async fn call_web_scraper(link: String, selectors: Vec<String>) {
    let web_scraper =
        web_scraper::WebScraper::new(link, selectors);
    web_scraper.scrape().await.unwrap();
}