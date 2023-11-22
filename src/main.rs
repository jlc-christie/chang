use std::io;
use anyhow::{bail, Result};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, read},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Frame, Terminal
};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

pub struct Chang<'a> {
    header: Paragraph<'a>,
    claims: Block<'a>,
    signature: Block<'a>,
}

impl Default for Chang<'_> {
    fn default() -> Self {
        let header = Paragraph::new("Some header").block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Header (^H) ")
        ).wrap(
            Wrap{ trim: false }
        );

        Chang {
            header,
            claims: Block::default().title(" Claims ").borders(Borders::ALL),
            signature: Block::default().title(" Signature ").borders(Borders::ALL),
        }
    }
}

impl<'a> Widget for Chang<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                    .as_ref(),
            )
            .split(area);

        self.header.render(chunks[0], buf);
        self.claims.render(chunks[1], buf);
        self.signature.render(chunks[2], buf);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = event_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                code => bail!(format!("unrecognised keycode: {:?}", code))
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    let chang = Chang::default();
    frame.render_widget(chang, frame.size());
}