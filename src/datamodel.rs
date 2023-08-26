use crate::{nyaa_scraper::get_body, tui::ui};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, widgets::ListState, Terminal};
use std::io;
use unhtml::FromHtml;

//-----scraper--------------------------------------------------------------------------------------------------------------

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

#[derive(FromHtml, Debug, Clone)]
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

#[derive(FromHtml, Debug, Clone)]
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

//-----tui------------------------------------------------------------------------------------------------------------------

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub show_popup: bool,
    pub entries: StatefulList<ListEntry>,
    pub body: Body,
}

impl<'a> App<'a> {
    pub async fn new() -> App<'a> {
        let data = get_body(None).await;
        App {
            titles: vec!["home", "nyaa", "downloads"],
            index: 0,
            show_popup: false,
            entries: StatefulList::with_items(data.entries.clone()),
            body: data,
        }
    }

    pub async fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('f') => self.show_popup = !self.show_popup,
                        key if key == KeyCode::Right
                            || key == KeyCode::Tab
                            || key == KeyCode::Char('l') =>
                        {
                            self.next_tab()
                        }
                        key if key == KeyCode::Left
                            || key == KeyCode::BackTab
                            || key == KeyCode::Char('h') =>
                        {
                            self.previous_tab()
                        }
                        key if key == KeyCode::Down || key == KeyCode::Char('k') => {
                            self.next_entry()
                        }
                        key if key == KeyCode::Up || key == KeyCode::Char('j') => {
                            self.previous_entry()
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn next_entry(&mut self) {
        self.entries.next()
    }

    pub fn previous_entry(&mut self) {
        self.entries.previous()
    }
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

    pub fn next(&mut self) {
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
