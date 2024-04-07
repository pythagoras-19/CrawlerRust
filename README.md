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

### Features

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
- GET https://crates.io/crates/rust-bert up and running for analysis