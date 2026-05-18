use std::io::{BufRead, stdin};

#[tokio::main]
async fn main() {
    aegis::send().await;

    println!("\nPress Enter to exit...");
    let mut iterator = stdin().lock().lines();
    let _ = iterator.next();
}
