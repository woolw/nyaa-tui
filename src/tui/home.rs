use crate::datamodel::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw_home<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>, app: &App) {
    let mut text = vec![text::Line::from("")];

    for controll_entry in app.controll_entries.iter() {
        text.push(text::Line::from(""));
        text.push(text::Line::from(vec![
            Span::styled(
                &controll_entry.title,
                Style::default()
                    .add_modifier(controll_entry.modifier)
                    .fg(controll_entry.color),
            ),
            Span::from(controll_entry.text.to_owned()),
        ]));
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}
