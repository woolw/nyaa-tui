use unhtml::FromHtml;

pub struct QueryParameters {
    pub filter: Dropdown,
    pub category: Dropdown,
    pub search_query: String,
    pub page: u32,
}

#[derive(FromHtml, Debug)]
pub struct Body {
    #[html(selector = "select[name = f]:nth-child(1) > option")]
    pub filter: Vec<Dropdown>,
    #[html(selector = "select[name = c]:nth-child(1) > option")]
    pub categories: Vec<Dropdown>,
    #[html(selector = ".default")]
    pub entries: Vec<ListEntry>,
    #[html(selector = ".pagination")]
    pub page_info: PageInfo,
}

#[derive(FromHtml, Debug)]
pub struct Dropdown {
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
