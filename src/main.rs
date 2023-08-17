use std::{env, io};

pub mod datamodel;
pub mod nyaa_scraper;
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // args handler
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let write_demo_files = args.iter().any(|x| x == "demo");

    // initial load of data
    let body = nyaa_scraper::extract_body(None, write_demo_files).await;

    tui::tui().await
}
