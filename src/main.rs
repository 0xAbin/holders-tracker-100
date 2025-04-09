use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod data;
mod getHolders;

fn main() {
    let addresses = data::get_eth_addresses();

    for token_address in addresses {
        let file_path = format!("BaseMainnet/{}.csv", &token_address[..42]);

        if Path::new(&file_path).exists() {
            println!("✅ Skipping (already processed): {}", token_address);
            continue;
        }

        println!("🚀 Processing token address: {}", token_address);
        getHolders::fetch_top_holders(token_address);

        sleep(Duration::from_secs(5));
    }
}
