use ratatui::widgets::*;
use serde::{Deserialize, Serialize};
use unhtml::FromHtml;

//-----find--------------------------------------------------------------------------------------------------------------

pub struct Dropdown {
    pub value: String,
    pub label: String,
}

impl Dropdown {
    pub fn get_filters() -> Vec<Dropdown> {
        vec![
            Dropdown {
                value: String::from("0"),
                label: String::from("No filter"),
            },
            Dropdown {
                value: String::from("1"),
                label: String::from("No remakes"),
            },
            Dropdown {
                value: String::from("2"),
                label: String::from("Trusted only"),
            },
        ]
    }
    pub fn get_categories() -> Vec<Dropdown> {
        vec![
            Dropdown {
                value: String::from("0_0"),
                label: String::from("All categories"),
            },
            Dropdown {
                value: String::from("1_0"),
                label: String::from("Anime"),
            },
            Dropdown {
                value: String::from("1_1"),
                label: String::from("Anime - AMV"),
            },
            Dropdown {
                value: String::from("1_2"),
                label: String::from("Anime - English"),
            },
            Dropdown {
                value: String::from("1_3"),
                label: String::from("Anime - Non-English"),
            },
            Dropdown {
                value: String::from("1_4"),
                label: String::from("Anime - Raw"),
            },
            Dropdown {
                value: String::from("2_0"),
                label: String::from("Audio"),
            },
            Dropdown {
                value: String::from("2_1"),
                label: String::from("Audio - Lossless"),
            },
            Dropdown {
                value: String::from("2_2"),
                label: String::from("Audio - Lossy"),
            },
            Dropdown {
                value: String::from("3_0"),
                label: String::from("Literature"),
            },
            Dropdown {
                value: String::from("3_1"),
                label: String::from("Literature - English"),
            },
            Dropdown {
                value: String::from("3_2"),
                label: String::from("Literature - Non-English"),
            },
            Dropdown {
                value: String::from("3_3"),
                label: String::from("Literature - Raw"),
            },
            Dropdown {
                value: String::from("4_0"),
                label: String::from("Live Action"),
            },
            Dropdown {
                value: String::from("4_1"),
                label: String::from("Live Action - English"),
            },
            Dropdown {
                value: String::from("4_2"),
                label: String::from("Live Action - Idol/PV"),
            },
            Dropdown {
                value: String::from("4_3"),
                label: String::from("Live Action - Non-English"),
            },
            Dropdown {
                value: String::from("4_4"),
                label: String::from("Live Action - Raw"),
            },
            Dropdown {
                value: String::from("5_0"),
                label: String::from("Pictures"),
            },
            Dropdown {
                value: String::from("5_1"),
                label: String::from("Pictures - Graphics"),
            },
            Dropdown {
                value: String::from("5_2"),
                label: String::from("Pictures - Photos"),
            },
            Dropdown {
                value: String::from("6_0"),
                label: String::from("Software"),
            },
            Dropdown {
                value: String::from("6_1"),
                label: String::from("Software - Apps"),
            },
            Dropdown {
                value: String::from("6_2"),
                label: String::from("Software - Games"),
            },
        ]
    }
}

pub struct QueryParameters {
    pub filter: StatefulList<Dropdown>,
    pub category: StatefulList<Dropdown>,
    pub search_query: SearchQuery,
    pub page: u32,
}

impl QueryParameters {
    pub fn new() -> QueryParameters {
        QueryParameters {
            filter: StatefulList::new_at_pos(Dropdown::get_filters(), 0),
            category: StatefulList::new_at_pos(Dropdown::get_categories(), 1),
            search_query: SearchQuery::new(),
            page: 1,
        }
    }
}

pub struct SearchQuery {
    pub search_string: String,
    pub is_insert_mode: bool,
    pub cursor_pos: usize,
}

impl SearchQuery {
    fn new() -> SearchQuery {
        SearchQuery {
            search_string: String::from(""),
            is_insert_mode: false,
            cursor_pos: 0,
        }
    }
}

//-----nyaa--------------------------------------------------------------------------------------------------------------

#[derive(FromHtml)]
pub struct Body {
    #[html(selector = ".default,.success,.danger")]
    pub entries: Vec<NyaaEntry>,
    #[html(selector = ".pagination > li:last-child > a", attr = "href")]
    pub next: Option<String>,
}

#[derive(FromHtml, Clone, Serialize, Deserialize)]
pub struct NyaaEntry {
    #[html(selector = ".category-icon", attr = "alt")]
    pub category: String,
    #[html(selector = "td:nth-child(2) > a:last-of-type", attr = "inner")]
    pub name: String,
    #[html(selector = "td:nth-child(3)")]
    pub download_links: DownloadLinks,
    #[html(selector = "td:nth-child(4)", attr = "inner")]
    pub size: String,
    #[html(selector = "td:nth-child(6)", attr = "inner")]
    pub seeder: u32,
    #[html(selector = "td:nth-child(7)", attr = "inner")]
    pub leecher: u32,
}

#[derive(FromHtml, Clone, Serialize, Deserialize)]
pub struct DownloadLinks {
    #[html(selector = "a:nth-child(1)", attr = "href")]
    pub torrent: String,
    #[html(selector = "a:nth-child(2)", attr = "href")]
    pub magnetic: String,
}

//-----tui------------------------------------------------------------------------------------------------------------------

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub params: QueryParameters,
    pub popup_state: PopupStates,
    pub nyaa_entries: StatefulList<NyaaEntry>,
    pub download_entries: StatefulList<NyaaEntry>,
    pub has_next: bool,
    pub exit_condition: ExitCondition,
}

pub enum ExitCondition {
    Quit,
    SaveList,
    Download,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn new_at_pos(items: Vec<T>, pos: usize) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };

        list.state.select(Some(pos));

        list
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            self.state = ListState::default();
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            self.state = ListState::default();
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub enum PopupStates {
    None,
    Find,
    AddDownload,
    RemoveDownload,
    NoneSelected,
    ExitCondition,
}
