use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashSet;

fn main() {
    let dir_path = "BaseMainet";
    let output_file = "Base_address_Token.csv";
    let mut all_addresses = HashSet::new();

  
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();


        if path.extension().map(|ext| ext == "csv").unwrap_or(false) {
            println!("📂 Processing {:?}", path.display());

            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);

          
            for (i, line) in reader.lines().enumerate() {
                if let Ok(record) = line {
                    if i == 0 { continue; } 
                    let parts: Vec<&str> = record.split(',').collect();
                    if parts.len() >= 2 {
                        all_addresses.insert(parts[1].to_string()); 
                    }
                }
            }
        }
    }

  
    let mut output = File::create(output_file).unwrap();
    for address in all_addresses {
        writeln!(output, "{}", address).unwrap();
    }

    println!("✅ All wallet addresses saved to '{}'", output_file);
}
