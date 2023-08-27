use crate::datamodel::{App, DownloadState};
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
            ListItem::new(text::Line::from(Span::styled(
                format!("{}", x.name),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(match x.download_state {
                        DownloadState::Queued => Color::White,
                        DownloadState::Downloading => Color::LightBlue,
                        DownloadState::Finished => Color::LightGreen,
                    }),
            )))
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

    match app.download_entries.state.selected() {
        Some(pos) => {
            let info = text::Line::from(vec![
                Span::raw("["),
                Span::styled(
                    format!(" size: {:?} |", app.download_entries.items[pos].size),
                    Style::default().fg(Color::LightBlue),
                ),
                match app.download_entries.items[pos].download_state {
                    DownloadState::Queued => Span::styled(
                        " Queued ",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    DownloadState::Downloading => Span::styled(
                        " Queued ",
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    ),
                    DownloadState::Finished => Span::styled(
                        " Finished ",
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    ),
                },
                Span::raw("]"),
            ]);
            let paragraph = Paragraph::new(info)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, area);
        }
        None => {}
    }
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
