use std::io;
use anyhow::{bail, Result};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, Event, read},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::prelude::*;
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui_textarea::{Input, Key, TextArea};

#[derive(Clone)]
struct Signature<'a> {
    text_area: TextArea<'a>,
    valid: bool,
}

impl<'a> Signature<'a> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }

    fn input(&mut self, input: Input) {
        match input {
            Input {
                key: Key::Char('v'),
                ctrl: true,
                ..
            } => self.valid = true,
            Input {
                key: Key::Char('b'),
                ctrl: true,
                ..
            } => self.valid = false,
            input => { self.text_area.input(input); },
        }

        self.update_block();
    }

    fn update_block(&mut self) {
        let title = if self.valid {
            Line::from(vec![
                Span::raw(" Decoding Key (^D) -"),
                Span::styled(" Valid ", Style::default().fg(Color::Green)),
            ])
        } else {
            Line::from(vec![
                Span::raw(" Decoding Key (^D) -"),
                Span::styled(" Invalid ", Style::default().fg(Color::Red)),
            ])
        };

       self.text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 185, 241)))
                .title(title),
        );
    }

    fn widget(&'a self) -> impl Widget + 'a {
        self.text_area.widget()
    }
}

impl Default for Signature<'_> {
    fn default() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 185, 241)))
                .title(Line::from(vec![
                    Span::raw(" Decoding Key (^D) -"),
                    Span::styled(" Invalid ", Style::default().fg(Color::Red)),
                ])),
        );
        text_area.set_cursor_line_style(Default::default());
        text_area.set_cursor_style(Default::default());

        Signature {
            text_area,
            valid: false,
        }
    }
}

#[derive(Clone)]
pub struct Chang<'a> {
    header: Paragraph<'a>,
    claims: Paragraph<'a>,
    signature: Signature<'a>,
}

impl Chang<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_input(&mut self, input: Input) {
        self.signature.input(input);
    }
}

impl Default for Chang<'_> {
    fn default() -> Self {
        let header = Paragraph::new("Some header").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(251, 1, 91)))
                .title(" Header (^H) ")
        ).wrap(
            Wrap{ trim: false }
        );

        let claims = Paragraph::new("Some claims").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(214, 58, 255)))
                .title(" Claims (^C) ")
        ).wrap(
            Wrap{ trim: false }
        );

        Chang {
            header,
            claims,
            signature: Signature::default(),
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
        terminal.draw(|frame| frame.render_widget(chang.clone(), frame.size()))?;

        let quit_inputs = [
            Input {
                key: Key::Esc,
                ctrl: false,
                alt: false,
                shift: false,
            },
            Input {
                key: Key::Char('c'),
                ctrl: true,
                alt: false,
                shift: false,
            },
        ];

        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(event) => {
                match Input::from(event) {
                    input if quit_inputs.contains(&input) => return Ok(()),
                    input => chang.process_input(input),
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}
