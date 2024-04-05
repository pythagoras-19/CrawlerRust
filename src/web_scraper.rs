use reqwest;
use scraper::{Html, Selector};
use crate::page_types::{get_paragraph_content, get_page_type_with_major_headings};

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

    // pub fn get_major_titles_and_headings(&self, page_type: &str) {
    //     let page_type_to_selectors = get_page_type_with_major_headings();
    //
    //     match page_type_to_selectors.get(page_type) {
    //         Some(selectors) => {
    //             for selector in selectors {
    //                 println!("{}", selector);
    //             }
    //         },
    //         None => println!("No selectors found for the given page type.")
    //     }
    // }

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
        let page_type_to_selectors = get_paragraph_content();

        let paragraph_content = match page_type_to_selectors.get(page_type) {
            Some(selectors) => selectors,
            None => return Ok(())
        };

        self.print_content_to_console(paragraph_content.to_vec(), fragment);
        Ok(())
    }

    pub fn print_content_to_console(&self, content: Vec<String>, fragment:scraper::Html) {
        for selector_str in content {
            let selector = Selector::parse((selector_str).as_str()).unwrap();
            for element in fragment.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                println!("{:?}", text);
            }
        }
    }
}