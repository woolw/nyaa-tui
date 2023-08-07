use std::{fs::File, io::Write};

use unhtml::FromHtml;

#[derive(FromHtml, Debug)]
pub struct Body {
    #[html(selector = "select[name = f]:nth-child(1) > option")]
    pub filter: Vec<Dropdowns>,
    #[html(selector = "select[name = c]:nth-child(1) > option")]
    pub categories: Vec<Dropdowns>,
    #[html(selector = ".default")]
    pub entries: Vec<ListEntry>,
    #[html(selector = ".pagination")]
    pub page_info: PageInfo,
}

#[derive(FromHtml, Debug)]
pub struct Dropdowns {
    #[html(attr = "value")]
    pub value: String,
    #[html(attr = "title")]
    pub title: String,
}

#[derive(FromHtml, Debug)]
pub struct ListEntry {
    #[html(selector = ".category-icon", attr = "alt")]
    pub category: String,
    #[html(selector = "td:nth-child(2)", attr = "inner")]
    pub name: String,
    #[html(selector = "td:nth-child(3)")]
    pub download_links: DownloadLinks,
    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub size: String,
    #[html(selector = "td:nth-child(5)", attr = "inner")]
    pub date: String,
    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub seeder: u32,
    #[html(selector = "td:nth-child(7)", attr = "inner")]
    pub leecher: u32,
    #[html(selector = "td:nth-child(8)", attr = "inner")]
    pub downloads: u32,
}

#[derive(FromHtml, Debug)]
pub struct DownloadLinks {
    #[html(selector = "a:nth-child(1)", attr = "href")]
    pub torrent: String,
    #[html(selector = "a:nth-child(2)", attr = "href")]
    pub magnetic: String,
}

#[derive(FromHtml, Debug)]
pub struct PageInfo {
    #[html(selector = "li:first-child > a", attr = "href")]
    pub previous: Option<String>,
    // has to be string, since sometimes the inner includes "(current)" inside a nested span
    #[html(selector = ".active > a", attr = "inner")]
    pub active: String,
    #[html(selector = "li:last-child > a", attr = "href")]
    pub next: Option<String>,
}

async fn get_html() -> String {
    let response = reqwest::get("https://nyaa.si/?f=0&c=0_0&q=made&p=14").await;

    let mut html = "".to_string();
    match response {
        Ok(it) => {
            html = it.text().await.unwrap();
            create_demo_files("demo.html", format!("{html:#}"));
        }
        Err(err) => println!("{:#?}", err),
    };

    html
}

pub async fn get_body() -> Body {
    let html = get_html().await;

    let t_body = Body::from_html(&html);

    match t_body {
        Ok(res) => {
            create_demo_files("result.json", format!("{res:?}"));
            res
        }
        Err(err) => panic!("t_bodies were not OK {:#?}", err),
    }
}

// soley for debugging
fn create_demo_files(filename: &str, data: String) {
    let file = File::create(format!("demo_files/{filename:#}"));
    match file {
        Ok(mut f) => match write!(f, "{:#?}", data) {
            Err(err) => println!("{err:#?}"),
            _ => {}
        },
        Err(err) => {
            println!("{err:#?}");
            panic!()
        }
    }
}
