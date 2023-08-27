use crate::datamodel::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw_nyaa<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>, app: &mut App) {
    let entries: Vec<ListItem> = app
        .nyaa_entries
        .items
        .iter()
        .map(|x| {
            ListItem::new(vec![text::Line::from(vec![Span::raw(format!(
                "{}",
                x.name
            ))])])
        })
        .collect();

    let nyaa_entries = List::new(entries)
        .block(block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(nyaa_entries, area, &mut app.nyaa_entries.state);

    match app.nyaa_entries.state.selected() {
        Some(pos) => {
            let info = text::Line::from(vec![
                Span::raw("["),
                Span::styled(
                    format!(" {:#} |", app.nyaa_entries.items[pos].category),
                    Style::default().fg(Color::LightMagenta),
                ),
                Span::styled(
                    format!(" size: {:#} |", app.nyaa_entries.items[pos].size),
                    Style::default().fg(Color::LightBlue),
                ),
                Span::styled(
                    format!(" seeders: {:#} |", app.nyaa_entries.items[pos].seeder),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(
                    format!(" leechers: {:#}", app.nyaa_entries.items[pos].leecher),
                    Style::default().fg(Color::Red),
                ),
                Span::raw(" ]"),
            ]);
            let paragraph = Paragraph::new(info)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, area);
        }
        None => {}
    }
}

pub fn draw_add_download<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    block: Block<'_>,
    area: Rect,
) {
    // we can use unwrap here, bcs we matched at an earlier step
    let pos = app.nyaa_entries.state.selected().unwrap();

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

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(paragraph, area);
}
