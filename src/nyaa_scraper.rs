pub mod nyaa_scraper {
    // #[macro_use]
    // extern crate unhtml;
    // extern crate unhtml_derive;
    // use unhtml_derive::{self, FromHtml};
    //
    // #[derive(FromHtml)]
    // #[html(selector = ".default")]
    // struct ListEntry {
    //     #[html(selector = ".category-icon", attr = "alt")]
    //     category: String,
    //
    //     name: String,
    //     link: (String, String),
    //     size: String,
    //     date: String,
    //     seeder: u32,
    //     leecher: u32,
    //     downloads: u32,
    // }

    pub fn get_html() {
        // TODO fetch data from nyaa.si
        // add filter and search functionality later on with enum

        let html = reqwest::blocking::get("https://nyaa.si/").unwrap().text();

        println!("{:#?}", html);
    }

    // pub fn get_list_entries() -> Vec<ListEntry> {
    // let html = get_html();
    // let listEntries = Vec::<ListEntry>::from_html(html);
    // match listEntries {
    // Ok(expr) => expr,
    // _ => panic!("listEntries were not OK"),
    // }
    // }
}
