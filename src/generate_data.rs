use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::{fs::File, io::Write};
use std::collections::HashSet;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let mut addresses: HashSet<String> = HashSet::new(); 
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Rust scraper)")
        .build()
        .unwrap();

    let token_url_template = env::var("ETHERSCAN_TOKEN_URL")
        .expect("ETHERSCAN_TOKEN_URL not set in .env");

    let max_token_pages: usize = env::var("TOKEN_PAGES")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("TOKEN_PAGES must be a valid number");

    let row_selector = Selector::parse("table tbody tr").unwrap();
    let link_selector = Selector::parse("a[href^='/token/0x']").unwrap();

    for page in 1..=max_token_pages {
        let url = token_url_template.replace("{page}", &page.to_string());
        println!("🌐 Scraping page {}: {}", page, url);

        let res = client.get(&url).send().unwrap().text().unwrap();
        let document = Html::parse_document(&res);

        for row in document.select(&row_selector) {
            if let Some(link) = row.select(&link_selector).next() {
                if let Some(href) = link.value().attr("href") {
                    if let Some(addr) = href.split("/token/").nth(1) {
                        addresses.insert(addr.to_string());
                    }
                }
            }
        }
    }

    let mut file = File::create("src/data.rs").unwrap();
    writeln!(file, "pub fn get_eth_addresses() -> Vec<&'static str> {{").unwrap();
    writeln!(file, "    vec![").unwrap();
    for addr in &addresses {
        writeln!(file, "        \"{}\",", addr).unwrap();
    }
    writeln!(file, "    ]").unwrap();
    writeln!(file, "}}").unwrap();

    println!("✅ Extracted {} unique token addresses and wrote to src/data.rs", addresses.len());
}
