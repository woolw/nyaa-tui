use ratatui::{prelude::*, widgets::*};

pub fn draw_home<B: Backend>(f: &mut Frame<B>, area: Rect, block: Block<'_>) {
    let text = vec![
        text::Line::from(""),
        text::Line::from(vec![Span::styled(
            "Welcome to nyaa-tui!",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "left: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Blue),
            ),
            Span::from("[h], [BACK_TAB], [LEFT_ARROW_KEY]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "down: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Green),
            ),
            Span::from("[j], [DOWN_ARROW_KEY]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "up: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Yellow),
            ),
            Span::from("[k], [UP_ARROW_KEY]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "right: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Red),
            ),
            Span::from("[l], [TAB], [RIGHT_ARROW_KEY]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "select: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Magenta),
            ),
            Span::from("[SPACE_BAR], [ENTER]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "load more nyaa entries: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::LightCyan),
            ),
            Span::from("[p]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "find: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Rgb(0xff, 0x8c, 0x00)),
            ),
            Span::from("[f]"),
        ]),
        text::Line::from(vec![
            Span::styled(
                "exit with download-options: ",
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::from("[q]"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}
