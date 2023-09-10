use crate::datamodel::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw_downloads<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>, app: &mut App) {
    let entries: Vec<ListItem> = app
        .download_entries
        .items
        .iter()
        .map(|x| ListItem::new(text::Line::from(x.name.to_string())))
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
                    format!(" {} |", app.download_entries.items[pos].category),
                    Style::default().fg(Color::LightMagenta),
                ),
                Span::styled(
                    format!(" size: {} |", app.download_entries.items[pos].size),
                    Style::default().fg(Color::LightBlue),
                ),
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

pub fn draw_remove_download<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: Block<'_>,
    area: Rect,
) {
    // we can use unwrap here, bcs we matched at an earlier step
    let pos = app.download_entries.state.selected().unwrap();

    let info = vec![
        text::Line::from(""),
        text::Line::from(vec![
            Span::raw("Do you want to remove "),
            Span::styled(
                format!("{}", app.download_entries.items[pos].name),
                Style::default().fg(Color::LightBlue),
            ),
            Span::raw(" from the download queue?"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled("[y]     ", Style::default().fg(Color::Red)),
            Span::styled("     [n]", Style::default().fg(Color::Green)),
        ]),
    ];
    let paragraph = Paragraph::new(info)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}
