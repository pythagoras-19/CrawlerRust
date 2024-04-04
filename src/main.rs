mod web_scraper;

#[tokio::main]
async fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter a website: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);
    call_web_scraper(s).await;
}

async fn call_web_scraper(link: String) {
    let web_scraper = web_scraper::WebScraper::new(link);
    web_scraper.call().await.unwrap();
}