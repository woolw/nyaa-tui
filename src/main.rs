mod nyaa_scraper;

#[tokio::main]
async fn main() {
    let mut entries = nyaa_scraper::get_list_entries().await;

    entries.sort_by_key(|x| x.seeder);

    for entry in entries.iter() {
        println!("{:#?}", entry);
    }
}
