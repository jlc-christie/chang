use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use tui_textarea::{Input, Key, TextArea};

#[derive(Clone)]
pub struct Signature<'a> {
    text_area: TextArea<'a>,
    valid: bool,
    active: bool,
}

impl<'a> Signature<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: Input) -> bool {
        let result = match input {
            Input {
                key: Key::Char('v'),
                ctrl: true,
                ..
            } => {
                self.valid = true;
                true
            },
            Input {
                key: Key::Char('b'),
                ctrl: true,
                ..
            } => {
                self.valid = false;
                true
            },
            input => self.text_area.input(input),
        };

        self.update_block();
        result
    }

    fn update_block(&mut self) {
        let title = if self.valid {
            Line::from(vec![
                Span::raw(" Decoding Key (^d) -"),
                Span::styled(" Valid ", Style::default().fg(Color::Green)),
            ])
        } else {
            Line::from(vec![
                Span::raw(" Decoding Key (^d) -"),
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

    pub fn widget(&'a self) -> impl Widget + 'a {
        self.text_area.widget()
    }
}

impl Default for Signature<'_> {
    fn default() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                // 96, 133, 144 @ 20% Luminance
                .border_style(Style::default().fg(Color::Rgb(0, 185, 241)))
                .title(Line::from(vec![
                    Span::raw(" Decoding Key (^d) -"),
                    Span::styled(" Invalid ", Style::default().fg(Color::Red)),
                ])),
        );
        text_area.set_cursor_line_style(Default::default());
        text_area.set_cursor_style(Default::default());

        Signature {
            text_area,
            valid: false,
            active: true,
        }
    }
}