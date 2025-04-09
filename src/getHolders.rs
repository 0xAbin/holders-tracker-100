use scraper::{Html, Selector};
use std::{fs::File, io::Write};
use std::time::Duration;
use std::thread::sleep;
use reqwest::blocking::Client;
use std::env;
use dotenv::dotenv;

pub fn fetch_top_holders(token_address: &str) {
    dotenv().ok(); 

    let base_url = env::var("ETHERSCAN_HOLDER_URL")
        .expect("ETHERSCAN_HOLDER_URL not set in .env");

    let max_pages: usize = env::var("PAGES")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("PAGES must be a valid number");

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; Rust scraper)")
        .build()
        .unwrap();

    let row_selector = Selector::parse("table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let span_selector = Selector::parse("span").unwrap();

    let mut rows = Vec::new();
    rows.push("Rank,Address,Quantity,Percentage".to_string());

    for page in 1..=max_pages {
        let url = base_url
            .replace("{address}", token_address)
            .replace("{page}", &page.to_string());

        println!("Fetching page {} for {}", page, token_address);
        let res = client.get(&url).send().unwrap().text().unwrap();
        let document = Html::parse_document(&res);

        for row in document.select(&row_selector) {
            let mut cells = row.select(&cell_selector);
        
            
            let rank_cell = cells.next();
            let address_cell = cells.next();
            let quantity_cell = cells.next();
            let percentage_cell = cells.next();
        
            if rank_cell.is_none() || address_cell.is_none() || quantity_cell.is_none() || percentage_cell.is_none() {
                continue;
            }
        
            let rank = rank_cell.unwrap().inner_html().trim().to_string();
        
            let address_link = address_cell
                .unwrap()
                .select(&link_selector)
                .next()
                .and_then(|link| link.value().attr("href"))
                .unwrap_or("")
                .split('=')
                .last()
                .unwrap_or("")
                .to_string();
        
            let quantity = quantity_cell
                .unwrap()
                .select(&span_selector)
                .next()
                .and_then(|s| s.value().attr("title"))
                .unwrap_or("")
                .to_string();
        
            let percentage = percentage_cell
                .unwrap()
                .select(&span_selector)
                .next()
                .and_then(|s| s.value().attr("title"))
                .unwrap_or("")
                .to_string();
        
            rows.push(format!("{},{},{},{}", rank, address_link, quantity, percentage));
        }
        

        sleep(Duration::from_secs(2)); 
    }

    std::fs::create_dir_all("ArbitrumMainet").unwrap();
    let filename = format!("ArbitrumMainet/{}.csv", &token_address[..42]);
    let mut file = File::create(&filename).unwrap();
    file.write_all(rows.join("\n").as_bytes()).unwrap();

    println!("✅ Saved top holders for {} to CSV", token_address);
}
