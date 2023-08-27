use ratatui::{prelude::*, widgets::*};
use unhtml::FromHtml;

//-----home------------------------------------------------------------------------------------------------------------------

pub struct ControllEntry {
    pub title: String,
    pub modifier: Modifier,
    pub color: Color,
    pub text: String,
}

impl ControllEntry {
    pub fn get_controlls() -> Vec<ControllEntry> {
        vec![
            ControllEntry {
                title: String::from("Welcome to nyaa-tui:"),
                modifier: Modifier::BOLD,
                color: Color::Reset,
                text: String::from(""),
            },
            ControllEntry {
                title: String::from("Controlls:"),
                modifier: Modifier::BOLD,
                color: Color::Reset,
                text: String::from(""),
            },
            ControllEntry {
                title: String::from("left:"),
                modifier: Modifier::ITALIC,
                color: Color::Blue,
                text: String::from(" [h], [BACK_TAB], [LEFT_ARROW_KEY]"),
            },
            ControllEntry {
                title: String::from("right:"),
                modifier: Modifier::ITALIC,
                color: Color::Red,
                text: String::from(" [l], [TAB], [RIGHT_ARROW_KEY]"),
            },
            ControllEntry {
                title: String::from("up:"),
                modifier: Modifier::ITALIC,
                color: Color::Yellow,
                text: String::from(" [j], [UP_ARROW_KEY]"),
            },
            ControllEntry {
                title: String::from("down:"),
                modifier: Modifier::ITALIC,
                color: Color::Green,
                text: String::from(" [k], [DOWN_ARROW_KEY]"),
            },
            ControllEntry {
                title: String::from("select:"),
                modifier: Modifier::ITALIC,
                color: Color::Magenta,
                text: String::from(" [ENTER], [SPACE_BAR]"),
            },
            ControllEntry {
                title: String::from("load more entries:"),
                modifier: Modifier::ITALIC,
                color: Color::LightCyan,
                text: String::from(" [p]"),
            },
            ControllEntry {
                title: String::from("find:"),
                modifier: Modifier::ITALIC,
                color: Color::Rgb(0xff, 0x8c, 0x00),
                text: String::from(" [f]"),
            },
            ControllEntry {
                title: String::from("exit:"),
                modifier: Modifier::ITALIC,
                color: Color::White,
                text: String::from(" [q]"),
            },
        ]
    }
}

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
    pub search_query: String,
    pub page: u32,
}

impl QueryParameters {
    pub fn new() -> QueryParameters {
        QueryParameters {
            filter: StatefulList::new_at_zero(Dropdown::get_filters()),
            category: StatefulList::new_at_zero(Dropdown::get_categories()),
            search_query: String::from(""),
            page: 1,
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

#[derive(FromHtml, Clone)]
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

#[derive(FromHtml, Clone)]
pub struct DownloadLinks {
    #[html(selector = "a:nth-child(1)", attr = "href")]
    pub torrent: String,
    #[html(selector = "a:nth-child(2)", attr = "href")]
    pub magnetic: String,
}

//-----downloads------------------------------------------------------------------------------------------------------------------

pub enum DownloadState {
    Queued,
    Downloading,
    Finished,
}

pub struct DownloadEntry {
    pub entry: NyaaEntry,
    pub download_state: DownloadState,
}

impl DownloadEntry {
    pub fn new(entry: NyaaEntry) -> DownloadEntry {
        DownloadEntry {
            entry,
            download_state: DownloadState::Queued,
        }
    }
}

//-----tui------------------------------------------------------------------------------------------------------------------

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub params: QueryParameters,
    pub popup_state: PopupStates,
    pub nyaa_entries: StatefulList<NyaaEntry>,
    pub download_entries: StatefulList<DownloadEntry>,
    pub has_next: bool,
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

    pub fn new_at_zero(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };

        list.state.select(Some(0));

        list
    }

    pub fn next(&mut self) {
        if self.items.len() <= 0 {
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
        if self.items.len() <= 0 {
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
}
