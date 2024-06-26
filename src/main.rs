use colored::*;
use log:: {info, LevelFilter};
use env_logger;
use tokio::time::Instant;

mod web_scraper;
mod parser;
mod page_types;
mod text_analysis;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    info!("Starting ==RUST CRAWLER==...");


    use std::io::{stdin,stdout,Write};
    let mut str =String::new();
    for _i in 0..3 {
        println!("{}", "<=====================================================================>".yellow());
    }
    print!("{}", "<===========> Enter a website: ".bright_green().bold());
    let _=stdout().flush();
    stdin().read_line(&mut str).expect("Did not enter a correct string");
    if let Some('\n')= str.chars().next_back() {
        str.pop();
    }
    if let Some('\r')= str.chars().next_back() {
        str.pop();
    }
    let selectors = parser::generate_selectors(&str).await.unwrap();
    println!("{}", "---- Web Scraping ----".green());
    call_web_scraper(str, selectors).await;
}

async fn call_web_scraper(link: String, selectors: Vec<String>) {
    let web_scraper =
        web_scraper::WebScraper::new(link, selectors);

    let _selectors = web_scraper.get_selectors();
    let _link = web_scraper.get_link();
    // web_scraper.scrape_entire_file().await.unwrap();
    println!("{}", "------------------ MAJOR TITLE AND HEADINGS ------------------".yellow());
    println!("{}", "----------- THIS CAN BE USED AS AN OUTLINE OF THE PAGE ----------------".yellow());
    if let Err(e) = web_scraper.scrape_major_titles_headings("html").await {
        println!("Error: {}", e);
    }
    println!("{}", "\n\n\n------------------ PARAGRAPH CONTENT ------------------".yellow());
    if let Err(e) = web_scraper.scrape_non_heading_content("html").await {
        println!("Error: {}", e);
    }

    println!("{}", "\n\n\n------------------ IMAGE CONTENT ------------------".yellow());
    if let Err(e) = web_scraper.scrape_image_content("html").await {
        println!("Error: {}", e);
    }
}