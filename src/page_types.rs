/**
This file returns only the major headings of a page type. IT DOES NOT RETURN ALL POSSIBLE HEADINGS.
The major headings are the headings that are most likely to be the main headings of a page.
**/

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