use std::env;

mod nyaa_scraper;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let write_demo_files: bool = args.iter().any(|x| x == "demo");
    let body = nyaa_scraper::extract_body(None, write_demo_files).await;
}
