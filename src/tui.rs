use self::{downloads::*, home::*, nyaa::*};
use crate::datamodel::{App, PopupStates};
use ratatui::{prelude::*, widgets::*};

mod downloads;
mod home;
mod nyaa;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().borders(Borders::ALL);

    // Tabs
    f.render_widget(block.clone(), size);
    let titles = app
        .titles
        .iter()
        .map(|t| Line::from(vec![t.reset()]))
        .collect();
    let tabs = Tabs::new(titles)
        .block(block.clone())
        .select(app.index)
        .style(Style::default())
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // content of the Main-Buffer
    match app.index {
        0 => draw_home(f, chunks[1], block.clone()),
        1 => draw_nyaa(f, chunks[1], block.clone(), app),
        2 => draw_downloads(f, chunks[1], block.clone(), app),
        _ => unreachable!(),
    };

    // popups
    let area = centered_rect(50, 25, f.size());
    match app.popup_state {
        PopupStates::Find => draw_find(f, app),
        PopupStates::AddDownload => draw_add_download(f, app, block, area),
        PopupStates::RemoveDownload => draw_remove_download(f, app, block, area),
        PopupStates::NoneSelected => draw_none_selected(f, block, area),
        PopupStates::None => {}
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

pub fn draw_find<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let info = vec![
        text::Line::from(""),
        text::Line::from(vec![
            Span::raw("[f]ilter: "),
            Span::styled(
                format!(
                    " {}",
                    app.params.filter.items[app.params.filter.state.selected().unwrap()].label
                ),
                Style::default().fg(Color::Gray),
            ),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::raw("[c]ategory: "),
            Span::styled(
                format!(
                    " {}",
                    app.params.category.items[app.params.category.state.selected().unwrap()].label
                ),
                Style::default().fg(Color::LightMagenta),
            ),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::raw("[s]earch: "),
            Span::styled(
                format!(" {}", app.params.search_query),
                Style::default().fg(Color::LightCyan),
            ),
        ]),
    ];

    let area = centered_rect(50, 30, f.size());
    let block = Block::default().borders(Borders::ALL).title("Find");
    let paragraph = Paragraph::new(info)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}

pub fn draw_none_selected<B: Backend>(f: &mut Frame<B>, block: Block<'_>, area: Rect) {
    let info = vec![
        text::Line::from(Span::raw(
            "you must first select something to for this action.",
        )),
        text::Line::from(""),
        text::Line::from(Span::raw("press q to exit this prompt.")),
    ];
    let paragraph = Paragraph::new(info)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}
