use std::collections::HashMap;

pub fn get_page_type_with_major_headings() -> HashMap<String, Vec<String>> {
    let mut page_type_to_selectors: HashMap<String, Vec<String>> = HashMap::new();
    page_type_to_selectors.insert("html".to_string(), vec!["h1".to_string(),
                                                           "h2".to_string(),
                                                           "h3".to_string(),
                                                           "h4".to_string(),
                                                           "h5".to_string(),
                                                           "h6".to_string(),
                                                           "title".to_string()]);
    // TODO: Add more page types and their MAJOR selectors
    page_type_to_selectors
}

pub fn get_page_type_with_paragraph_content() -> HashMap<String, Vec<String>> {
    let mut page_type_to_selectors: HashMap<String, Vec<String>> = HashMap::new();
    page_type_to_selectors.insert("html".to_string(), vec!["p".to_string()]);

    page_type_to_selectors
}

pub fn get_page_type_with_image_content() -> HashMap<String, Vec<String>> {
    let mut page_type_to_selectors: HashMap<String, Vec<String>> = HashMap::new();
    page_type_to_selectors.insert("html".to_string(), vec!["img".to_string()]);

    page_type_to_selectors
}

pub fn get_page_type_with_next_page_link() -> HashMap<String, String> {
    let page_type_selectors: HashMap<String, String> = HashMap::new();
    //TODO: FINISH ME

    page_type_selectors
}