use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let log_path = dirs::home_dir().unwrap().join("server.log");
    let mut counter = 0;

    loop {
        let message = format!("Message {}\n", counter);
        
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&log_path)
            .expect("Failed to open log file");

        file.write_all(message.as_bytes()).expect("Failed to write to log file");
        
        println!("Server wrote: {}", message.trim());
        
        counter += 1;
        sleep(Duration::from_secs(1));
    }
}