use std::collections::{HashSet};
use jsonwebtoken::Algorithm;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use serde_json::Value;
use tui_textarea::{Input, Key, TextArea};

#[derive(Clone)]
pub struct Signature<'a> {
    text_area: TextArea<'a>,
    jwt: String,
    valid: bool,
    active: bool,
}

impl<'a> Signature<'a> {
    #[allow(dead_code)]
    pub fn new(jwt: String) -> Self {
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
            jwt,
            valid: false,
            active: true,
        }
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

        self.validate_signature();
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

    fn validate_signature(&mut self) {
        let lines = self.text_area.clone().into_lines();
        let decoding_key = lines.join("\n");
        let mut validation = jsonwebtoken::Validation::new(Algorithm::HS256);
        validation.required_spec_claims = HashSet::new();
        validation.validate_exp = false;

        let token_message = jsonwebtoken::decode::<Value>(
            self.jwt.as_str(),
            &jsonwebtoken::DecodingKey::from_secret(decoding_key.as_bytes()),
            &validation,
        );
        // TODO(jlc-christie): display error message to user somehow?
        self.valid = token_message.is_ok();
    }
}
