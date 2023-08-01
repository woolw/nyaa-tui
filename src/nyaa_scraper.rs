use unhtml::FromHtml;

// has to be done with nested structs, bcs topLevel Vec doesn't work
#[derive(FromHtml, Debug)]
struct TBody {
    #[html(selector = ".default")]
    html_entries: Vec<ListEntry>,
}

#[derive(FromHtml, Debug)]
pub struct ListEntry {
    #[html(selector = ".category-icon", attr = "alt")]
    category: String,
    #[html(selector = "td:nth-child(2)", attr = "inner")]
    name: String,
    #[html(selector = "td:nth-child(3)")]
    download_links: DownloadLinks,
    #[html(selector = "td:nth-child(4)", attr = "inner")]
    size: String,
    #[html(selector = "td:nth-child(5)", attr = "inner")]
    date: String,
    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub seeder: u32,
    #[html(selector = "td:nth-child(7)", attr = "inner")]
    leecher: u32,
    #[html(selector = "td:nth-child(8)", attr = "inner")]
    downloads: u32,
}

#[derive(FromHtml, Debug)]
pub struct DownloadLinks {
    #[html(selector = "a:nth-child(1)", attr = "href")]
    torrent: String,
    #[html(selector = "a:nth-child(2)", attr = "href")]
    magnetic: String,
}

async fn get_html() -> String {
    let response = reqwest::get("https://nyaa.si/").await;

    let mut html: String = "".to_string();
    match response {
        Ok(it) => html = it.text().await.unwrap(),
        Err(err) => println!("{:#?}", err),
    };

    //println!("{:#?}", html);

    html
}

pub async fn get_list_entries() -> Vec<ListEntry> {
    let html = get_html().await;

    let t_body = TBody::from_html(&html);

    match t_body {
        Ok(res) => {
            println!("{:#?}", res);
            res.html_entries
        }
        Err(err) => panic!("t_bodies were not OK {:#?}", err),
    }
}
