use crate::datamodel::App;
use ratatui::{prelude::*, widgets::*};

// tabs
// gauge
// list of queue

pub fn draw_downloads<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>, app: &mut App) {
    let entries: Vec<ListItem> = app
        .download_entries
        .items
        .iter()
        .map(|x| {
            ListItem::new(vec![text::Line::from(vec![Span::raw(format!(
                "{}",
                x.name
            ))])])
        })
        .collect();

    let download_entries = List::new(entries)
        .block(block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(download_entries, area, &mut app.download_entries.state);
}

// fn draw_gauge<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>, app: &mut App) {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(2),
//                 Constraint::Length(3),
//                 Constraint::Length(1),
//             ]
//             .as_ref(),
//         )
//         .margin(1)
//         .split(area);
//     let block = Block::default().borders(Borders::ALL).title("Downloading");
//     f.render_widget(block, area);

//     let line_gauge = LineGauge::default()
//         .block(Block::default().title("Downloading"))
//         .gauge_style(Style::default().fg(Color::LightGreen))
//         .line_set(if app.enhanced_graphics {
//             symbols::line::THICK
//         } else {
//             symbols::line::NORMAL
//         })
//         .ratio(app.progress);
//     f.render_widget(line_gauge, chunks[0]);
// }
