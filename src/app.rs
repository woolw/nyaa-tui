use crate::{
    datamodel::{App, DownloadEntry, NyaaEntry, PopupStates, QueryParameters, StatefulList},
    scraper::get_body,
    tui::ui,
};
use crossterm::event::{self, *};
use ratatui::prelude::*;
use std::io;

impl<'a> App<'a> {
    pub async fn new() -> App<'a> {
        let new_params = QueryParameters::new();
        let data = get_body(&new_params).await;
        App {
            titles: vec!["home", "nyaa", "downloads"],
            index: 0,
            params: new_params,
            popup_state: PopupStates::None,
            nyaa_entries: StatefulList::with_items(data.entries),
            download_entries: StatefulList::with_items(vec![]),
            has_next: data.next.is_some(),
        }
    }

    pub async fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // split controlls up by popup state
                    match self.popup_state {
                        PopupStates::None => match key.code {
                            key if (key == KeyCode::Left
                                || key == KeyCode::BackTab
                                || key == KeyCode::Char('h')) =>
                            {
                                self.previous_tab()
                            }
                            key if (key == KeyCode::Right
                                || key == KeyCode::Tab
                                || key == KeyCode::Char('l')) =>
                            {
                                self.next_tab()
                            }
                            key if (key == KeyCode::Up || key == KeyCode::Char('j')) => {
                                self.previous_entry()
                            }
                            key if (key == KeyCode::Down || key == KeyCode::Char('k')) => {
                                self.next_entry()
                            }
                            key if (key == KeyCode::Enter || key == KeyCode::Char(' ')) => {
                                self.select_entry()
                            }
                            KeyCode::Char('p') => self.load_next_page().await,
                            KeyCode::Char('f') => self.popup_state = PopupStates::Find,
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        PopupStates::Find => match key.code {
                            KeyCode::Char('f') => todo!(),
                            KeyCode::Char('c') => todo!(),
                            KeyCode::Char('s') => todo!(),
                            key if (key == KeyCode::Enter || key == KeyCode::Char(' ')) => {
                                todo!()
                            }
                            KeyCode::Char('q') => self.popup_state = PopupStates::None,
                            _ => {}
                        },
                        PopupStates::AddDownload => match key.code {
                            KeyCode::Char('y') => match self.nyaa_entries.state.selected() {
                                Some(pos) => {
                                    self.add_download(self.nyaa_entries.items[pos].clone());
                                    self.popup_state = PopupStates::None;
                                }
                                None => {}
                            },
                            KeyCode::Char('n') => self.popup_state = PopupStates::None,
                            _ => {}
                        },
                        PopupStates::RemoveDownload => match key.code {
                            KeyCode::Char('y') => match self.download_entries.state.selected() {
                                Some(pos) => {
                                    self.remove_download(pos);
                                    self.popup_state = PopupStates::None;
                                }
                                None => {}
                            },
                            KeyCode::Char('n') => self.popup_state = PopupStates::None,
                            _ => {}
                        },
                        PopupStates::NoneSelected => match key.code {
                            KeyCode::Char('q') => self.popup_state = PopupStates::None,
                            _ => {}
                        },
                    }
                }
            }
        }
    }

    fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    fn next_entry(&mut self) {
        match self.index {
            1 => self.nyaa_entries.next(),
            2 => self.download_entries.next(),
            _ => {}
        }
    }

    fn previous_entry(&mut self) {
        match self.index {
            1 => self.nyaa_entries.previous(),
            2 => self.download_entries.previous(),
            _ => {}
        }
    }

    fn select_entry(&mut self) {
        match self.index {
            1 => match self.nyaa_entries.state.selected() {
                Some(_) => self.popup_state = PopupStates::AddDownload,
                None => self.popup_state = PopupStates::NoneSelected,
            },
            2 => match self.download_entries.state.selected() {
                Some(_) => self.popup_state = PopupStates::RemoveDownload,
                None => self.popup_state = PopupStates::NoneSelected,
            },
            _ => {}
        }
    }

    async fn load_next_page(&mut self) {
        match self.index {
            1 => {
                if self.has_next {
                    self.params.page += 1;
                    let mut next_page = get_body(&self.params).await;
                    self.has_next = next_page.next.is_some();
                    self.nyaa_entries.items.append(&mut next_page.entries)
                }
            }
            _ => {}
        }
    }

    fn add_download(&mut self, entry: NyaaEntry) {
        if !self.download_entries.items.iter().any(|x| {
            x.entry.download_links.magnetic == entry.download_links.magnetic
                || x.entry.download_links.torrent == entry.download_links.torrent
        }) {
            self.download_entries.items.push(DownloadEntry::new(entry));
        }
    }

    fn remove_download(&mut self, pos: usize) {
        let _ = self.download_entries.items.remove(pos);
        self.download_entries.next();
    }
}
