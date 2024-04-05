# CrawlerRust

## Bugs
- `torch-sys` bug
  - run `cargo build` to see the error
  - **Solutions:**
    - option 1: Grind and fix the error ( 1hr )
    - option 2: Remove the `torch-sys` dependency (10min)
    - option 3: Use a different crate that doesn't have this issue (2 hrs)
    - option 4: step away from this dependency and focus smaller (like writing our own algorithm to summarize articles) (3 hrs)
## Goals Summary
- Summarize web pages quickly and easily using Rust.
- Stretch: Summarize multiple web pages in parallel

## Features

- Send HTTP GET requests using the `reqwest` crate.
- Parse HTML responses using the `scraper` crate.
- Extract specific data from the parsed HTML.

### Common links to use 
- https://www.rust-lang.org/

## Prerequisites

- Rust 1.54.0 or later
- Cargo 1.54.0 or later

## Dependencies

- `reqwest` for sending HTTP requests.
- `scraper` for parsing HTML and extracting data.
- `tokio` for asynchronous programming.

## Usage

1. Clone the repository.
2. Navigate to the project directory.
3. Run the project using the command `cargo run`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)

# TODO
## Features
Add more page types and their MAJOR selectors

Dynamic Scraping Engine: Build a scraping engine capable of handling different website layouts and structures. Use Rust's asynchronous programming features to handle multiple scraping tasks concurrently, efficiently managing resources and network calls.

Content Detection and Extraction: Implement advanced content detection algorithms to extract meaningful information from web pages, such as article texts, authors, publication dates, and images. Consider using machine learning models for more accurate content categorization and extraction.

Sentiment Analysis: Integrate a sentiment analysis library or API to assess the tone and sentiment of the collected content. This can provide insights into public opinion on various topics or events.

Data Storage and Indexing: Store the scraped data in a database with efficient indexing to support fast searches and queries. Explore Rust's ecosystem for interacting with databases, such as Diesel for SQL databases or mongodb for NoSQL options.

Web Interface and API: Develop a web interface using a Rust web framework (e.g., Rocket, Actix-web) that allows users to search for news by topic, sentiment, or source. Additionally, provide an API for programmatic access to the data.

Real-time Updates and Notifications: Implement a system to notify users of new articles or significant changes in sentiment for chosen topics. This could involve web sockets for real-time updates or a scheduled task system for periodic notifications.

Customizable Scraping Rules: Allow users or administrators to add new sources or modify scraping rules through a user-friendly interface. This requires designing a flexible and extensible architecture for your scraping engine.

Compliance and Ethics: Ensure the project complies with website terms of service, robots.txt rules, and ethical guidelines for web scraping. Implement rate limiting and user-agent identification to responsibly interact with websites.

## Technical Considerations
Concurrency and Error Handling: Leverage Rust's async/await syntax and error handling paradigms to manage concurrent scraping tasks and handle network or parsing errors gracefully.

Data Processing: Utilize Rust's powerful type system and pattern matching to process and analyze the scraped data. Explore crates like serde for serialization/deserialization and regex for text processing.

Machine Learning Integration: For sentiment analysis, either integrate an existing Rust machine learning library (if available and suitable) or interface with external machine learning services or APIs.

Frontend Development: For the web interface, you can either build it in Rust using a WebAssembly framework like Yew or Seed, or use traditional web technologies (HTML/CSS/JavaScript) and communicate with the Rust backend via REST or GraphQL APIs.

