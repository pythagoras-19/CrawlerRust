use colored::*;
use reqwest;
use scraper::{Html, Selector};
use crate::page_types::{get_page_type_with_paragraph_content,
                        get_page_type_with_major_headings,
                        get_page_type_with_image_content};

pub(crate) struct WebScraper {
    link: String,
    selectors: Vec<String>,
}

impl WebScraper {
    pub fn get_selectors(&self) -> &Vec<String> {
        &self.selectors
    }

    pub fn get_link(&self) -> &String {
        &self.link
    }

    pub fn new(link: String, selectors: Vec<String>) -> Self {
        WebScraper {
            link,
            selectors,
        }
    }
    #[allow(dead_code)]
    pub async fn scrape_entire_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);

        for selector_str in &self.selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in fragment.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                println!("{:?}", text);
            }
        }
        Ok(())
    }

    pub async fn scrape_major_titles_headings(&self, page_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);
        let page_type_to_selectors = get_page_type_with_major_headings();

        let major_headings = match page_type_to_selectors.get(page_type) {
            Some(selectors) => selectors,
            None => return Ok(())
        };

        self.print_content_to_console(major_headings.to_vec(), fragment);
        Ok(())
    }

    pub async fn scrape_non_heading_content(&self, page_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);
        let page_type_to_selectors = get_page_type_with_paragraph_content();

        let paragraph_content = match page_type_to_selectors.get(page_type) {
            Some(selectors) => selectors,
            None => return Ok(())
        };

        self.print_content_to_console(paragraph_content.to_vec(), fragment);
        Ok(())
    }

    pub async fn scrape_image_content(&self, page_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.link).await?.text().await?;
        let fragment = Html::parse_document(&body);
        let page_type_to_selectors = get_page_type_with_image_content();

        let image_content = match page_type_to_selectors.get(page_type) {
            Some(selectors) => selectors,
            None => return Ok(())
        };

        self.print_image_content_to_console(image_content.to_vec(), fragment);
        Ok(())
    }

    /**
        TODO: ADD Pagination: Follow links and scrape that content also
    **/

    pub fn print_content_to_console(&self, content: Vec<String>, fragment: scraper::Html) {
        for selector_str in content {
            let selector = Selector::parse((selector_str).as_str()).unwrap();
            for element in fragment.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                println!("{:?}", text);
            }
        }
    }

    pub fn print_image_content_to_console(&self, content: Vec<String>, fragment: scraper::Html) {
        for selector_str in content {
            let selector = Selector::parse((selector_str).as_str()).unwrap();
            for element in fragment.select(&selector) {
                if selector_str == "img" {
                    if let Some(src) = element.value().attr("src") {
                        /*
                            ERROR HANDLING:
                            Only executes if element.value().attr("src")
                            is Some(value).If it's None, the code here will be skipped!
                        */
                        println!("{:?}", src);
                    } else {
                        println!("{}", "No src attribute found!".red());
                    }
                } else {
                    println!("{}", "No image to show!".red());
                }
            }
        }
    }
}
