use self::{downloads::draw_downloads, home::draw_home, nyaa::draw_nyaa};
use crate::datamodel::{App, Popups};
use crossterm::{
    event::*,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self};

mod downloads;
mod home;
mod nyaa;

pub async fn tui() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new().await;
    let res = app.run(&mut terminal).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default();

    // Tabs
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| Line::from(vec![t.reset()]))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(app.index)
        .style(Style::default())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // Main-Buffer
    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // content of the Main-Buffer
    match app.index {
        0 => draw_home(f, chunks[1], main_block, app),
        1 => draw_nyaa(f, chunks[1], main_block, app),
        2 => draw_downloads(f, chunks[1], main_block, app),
        _ => unreachable!(),
    };

    // search popup
    match app.popup {
        Popups::Find => todo!(),
        Popups::AddDownload => draw_select_for_download(f, app),
        Popups::RemoveDownload => todo!(),
        Popups::None => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

//-----Popups----------------------------------------------------------------------------------------------------------------------------------------------------

pub fn draw_select_for_download<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default().borders(Borders::ALL);
    let area = centered_rect(50, 35, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block.clone(), area);

    match app.nyaa_entries.state.selected() {
        Some(pos) => {
            let info = vec![
                text::Line::from(""),
                text::Line::from(vec![
                    Span::raw("Do you want to download "),
                    Span::styled(
                        format!("{:?}", app.nyaa_entries.items[pos].name),
                        Style::default().fg(Color::LightBlue),
                    ),
                    Span::raw("?"),
                ]),
                text::Line::from(""),
                text::Line::from(vec![
                    Span::styled("[y]     ", Style::default().fg(Color::Green)),
                    Span::styled("     [n]", Style::default().fg(Color::Red)),
                ]),
            ];
            let paragraph = Paragraph::new(info)
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, area);
        }
        None => {
            let _ = none_selected_popup(f, block, area);
        }
    };
}

pub fn none_selected_popup<B: Backend>(f: &mut Frame<B>, block: Block<'_>, area: Rect) {
    let info = vec![
        text::Line::from(Span::raw(
            "you must first select something to for this action.",
        )),
        text::Line::from(Span::raw("")),
        text::Line::from(Span::raw("press q to exit this prompt.")),
    ];
    let paragraph = Paragraph::new(info)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
