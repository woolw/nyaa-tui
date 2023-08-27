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
                    format!(" size: {:?} |", app.nyaa_entries.items[pos].size),
                    Style::default().fg(Color::LightBlue),
                ),
                Span::styled(
                    format!(" seeders: {:?} |", app.nyaa_entries.items[pos].seeder),
                    Style::default().fg(Color::Green),
                ),
                Span::styled(
                    format!(" leechers: {:?}", app.nyaa_entries.items[pos].leecher),
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
