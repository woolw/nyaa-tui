mod nyaa_scraper;

#[tokio::main]
async fn main() {
    let body = nyaa_scraper::extract_body(None).await;

    println!("{:#?}", body);
}
