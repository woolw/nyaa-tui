mod nyaa_scraper;

#[tokio::main]
async fn main() {
    let body = nyaa_scraper::get_body().await;

    println!("{:#?}", body);
}
