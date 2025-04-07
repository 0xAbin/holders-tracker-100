mod data;
mod getHolders;

fn main() {
    let addresses = data::get_eth_addresses();

    for token_address in addresses {
        println!("Processing token address: {}", token_address);
        getHolders::fetch_top_holders(token_address);
    }
}