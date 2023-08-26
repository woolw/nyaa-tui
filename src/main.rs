use std::io;

pub mod datamodel;
pub mod nyaa_scraper;
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    tui::tui().await
}
