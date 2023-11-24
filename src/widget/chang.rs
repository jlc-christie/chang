use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui_textarea::Input;
use crate::widget::Signature;

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