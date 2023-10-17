use crate::{
    datamodel::{App, NyaaEntry, PopupStates, QueryParameters, StatefulList},
    scraper::get_body,
    tui::ui,
};
use crossterm::event::{self, *};
use ratatui::{prelude::*, widgets::ListState};
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

    pub async fn run<B: Backend>(
        mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<Vec<NyaaEntry>>> {
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
                            key if (key == KeyCode::Up || key == KeyCode::Char('k')) => {
                                self.previous_entry()
                            }
                            key if (key == KeyCode::Down || key == KeyCode::Char('j')) => {
                                self.next_entry()
                            }
                            key if (key == KeyCode::Enter || key == KeyCode::Char(' ')) => {
                                self.select_entry()
                            }
                            KeyCode::Char('p') => self.append_next_page().await,
                            KeyCode::Char('f') => self.popup_state = PopupStates::Find,
                            KeyCode::Char('d') => return Ok(Some(self.download_entries.items)),
                            KeyCode::Char('q') => return Ok(None),
                            _ => {}
                        },
                        PopupStates::Find => match self.params.search_query.is_insert_mode {
                            true => match key.code {
                                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                                KeyCode::Backspace => self.delete_char_left(),
                                KeyCode::Delete => self.delete_char_right(),
                                KeyCode::Left => self.move_cursor_left(),
                                KeyCode::Right => self.move_cursor_right(),
                                KeyCode::Esc => {
                                    self.params.search_query.is_insert_mode = false;
                                }
                                KeyCode::Enter => {
                                    self.params.search_query.is_insert_mode = false;
                                    self.reload().await;
                                    self.popup_state = PopupStates::None
                                }
                                _ => {}
                            },
                            false => match key.code {
                                KeyCode::Char('f') => self.params.filter.next(),
                                KeyCode::Char('c') => self.params.category.next(),
                                KeyCode::Char('i') => {
                                    self.params.search_query.is_insert_mode = true
                                }
                                key if (key == KeyCode::Enter || key == KeyCode::Char(' ')) => {
                                    self.reload().await;
                                    self.popup_state = PopupStates::None
                                }
                                KeyCode::Char('q') => self.popup_state = PopupStates::None,
                                _ => {}
                            },
                        },
                        PopupStates::AddDownload => match key.code {
                            KeyCode::Char('y') => match self.nyaa_entries.state.selected() {
                                Some(pos) => {
                                    self.add_download(self.nyaa_entries.items[pos].clone());
                                    self.popup_state = PopupStates::None;
                                }
                                None => {}
                            },
                            key if (key == KeyCode::Char('n') || key == KeyCode::Esc) => {
                                self.popup_state = PopupStates::None
                            }
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
                            key if (key == KeyCode::Char('n') || key == KeyCode::Esc) => {
                                self.popup_state = PopupStates::None
                            }
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

    async fn reload(&mut self) {
        self.params.page = 1;
        let new_page = get_body(&self.params).await;
        self.has_next = new_page.next.is_some();
        self.nyaa_entries.items = new_page.entries;
        self.nyaa_entries.state = ListState::default();

        self.index = 1
    }

    async fn append_next_page(&mut self) {
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
            x.download_links.magnetic == entry.download_links.magnetic
                || x.download_links.torrent == entry.download_links.torrent
        }) {
            self.download_entries.items.push(entry);
        }
    }

    fn remove_download(&mut self, pos: usize) {
        let _ = self.download_entries.items.remove(pos);
        self.download_entries.next();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.params.search_query.cursor_pos.saturating_sub(1);
        self.params.search_query.cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.params.search_query.cursor_pos.saturating_add(1);
        self.params.search_query.cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.params
            .search_query
            .search_string
            .insert(self.params.search_query.cursor_pos, new_char);

        self.move_cursor_right();
    }

    fn delete_char_left(&mut self) {
        if self.params.search_query.cursor_pos != 0 {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.params.search_query.cursor_pos;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self
                .params
                .search_query
                .search_string
                .chars()
                .take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self
                .params
                .search_query
                .search_string
                .chars()
                .skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.params.search_query.search_string =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn delete_char_right(&mut self) {
        if self.params.search_query.cursor_pos <= self.params.search_query.search_string.len() {
            let current_index = self.params.search_query.cursor_pos;
            let from_current_index_to_right = current_index + 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self
                .params
                .search_query
                .search_string
                .chars()
                .take(current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self
                .params
                .search_query
                .search_string
                .chars()
                .skip(from_current_index_to_right);

            self.params.search_query.search_string =
                before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.params.search_query.search_string.len())
    }
}
