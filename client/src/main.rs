use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let log_path = dirs::home_dir().unwrap().join("server.log");

    loop {
        if let Ok(file) = File::open(&log_path) {
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(content) = line {
                    println!("Client received: {}", content);
                }
            }
        }

        sleep(Duration::from_secs(1));
    }
}