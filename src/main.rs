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
use tui_textarea::TextArea;

#[derive(Clone)]
pub struct Chang<'a> {
    header: Paragraph<'a>,
    claims: Paragraph<'a>,
    signature: TextArea<'a>,
}

impl Chang<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(k) => match k.code {
                KeyCode::Char(char) => {
                    self.signature.insert_char(char);
                }
                KeyCode::Backspace => {
                    self.signature.delete_char();
                }
                _ => {}
            },
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
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

        let claims = Paragraph::new("Some claims").block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Claims (^C) ")
        ).wrap(
            Wrap{ trim: false }
        );

        let signature_block = Block::default()
            .borders(Borders::ALL)
            .title(" Signature (^S) ");
        let mut signature = TextArea::default();
        signature.set_block(signature_block);

        Chang {
            header,
            claims,
            signature,
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
        self.signature.widget().render(chunks[2], buf);
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
    let mut chang = Chang::default();

    loop {
        terminal.draw(|frame| ui(frame, &chang))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Char(_) | KeyCode::Backspace => {
                    chang.process_event(Event::Key(key));
                }
                code => bail!(format!("unrecognised keycode: {:?}", code))
            }
        }
    }
}

fn ui(frame: &mut Frame, chang: &Chang, ) {
    frame.render_widget(chang.clone(), frame.size());
}