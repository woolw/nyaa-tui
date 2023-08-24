use crate::datamodel::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io;

pub async fn tui() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["home", "nyaa", "downloads"],
            index: 0,
            show_popup: false,
        }
    }

    pub fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('f') => app.show_popup = !app.show_popup,
                    key if assert_next_tab(key) => app.next_tab(),
                    key if assert_previous_tab(key) => app.previous_tab(),
                    _ => {}
                }
            }
        }
    }
}

fn assert_next_tab(key: KeyCode) -> bool {
    key == KeyCode::Right || key == KeyCode::Tab || key == KeyCode::Char('l')
}

fn assert_previous_tab(key: KeyCode) -> bool {
    key == KeyCode::Left || key == KeyCode::BackTab || key == KeyCode::Char('h')
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
        .map(|t| Line::from(vec![t.reset()])) // i do not know why it doesn´t accept only t
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
        0 => draw_home(f, chunks[1], main_block),
        1 => {} // TODO draw list from body entries
        2 => {} // TODO show downloads
        _ => unreachable!(),
    };

    // search popup
    if app.show_popup {
        let block = Block::default().title("search nyaa").borders(Borders::ALL);
        let area = centered_rect(50, 40, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn draw_home<B>(f: &mut Frame<B>, area: Rect, block: Block<'_>)
where
    B: Backend,
{
    let text = vec![
        text::Line::from(""),
        text::Line::from(""),
        text::Line::from(""),
        text::Line::from(Span::styled(
            "Welcome to nyaa-tui",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        text::Line::from(""),
        text::Line::from(Span::styled(
            "Controlls:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "left:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Blue),
            ),
            Span::from(" [h], [BACK_TAB], [LEFT_ARROW_KEY]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "right:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Red),
            ),
            Span::from(" [l], [TAB], [RIGHT_ARROW_KEY]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "up:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Yellow),
            ),
            Span::from(" [j], [UP_ARROW_KEY]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "down:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Green),
            ),
            Span::from(" [k], [DOWN_ARROW_KEY]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "select:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Magenta),
            ),
            Span::from(" [ENTER]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "find:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Rgb(0xff, 0x8c, 0x00)),
            ),
            Span::from(" [f]"),
        ]),
        text::Line::from(""),
        text::Line::from(vec![
            Span::styled(
                "exit:",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::White),
            ),
            Span::from(" [q]"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}
