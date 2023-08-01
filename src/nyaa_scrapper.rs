#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
#[html(selector = ".default")]
struct listEntry {
    #[html(selector = ".category-icon", attr = "alt")]
    category: String,

    name: String,
    link: (String, String),
    size: String,
    date: String,
    seeder: u32,
    leecher: u32,
    downloads: u32,
}

fn get_html() -> str {
    // TODO fetch data from nyaa.si
    // add filter and search functionality later on with enum
    ""
}

pub fn get_list_entries() -> Vec<listEntry> {
    let html = get_html();
    let listEntries = Vec::<listEntry>::fromHtml(html);
}
