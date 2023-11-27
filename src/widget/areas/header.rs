use tui_textarea::TextArea;
use anyhow::Result;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

#[derive(Clone)]
pub struct Header<'a> {
    text_area: TextArea<'a>
}

impl<'a> Header<'a> {
    pub fn new(text: impl Into<Vec<String>>) -> Result<Self> {
        let mut text_area = TextArea::new(text.into());
        // 150, 100, 118 @ 20% Luminance
        text_area.set_line_number_style(Style::default().fg(Color::Rgb(251, 1, 91)));
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(251, 1, 91)))
            .title(" Header (^h) "));

        Ok(
            Header {
                text_area,
            }
        )
    }

    pub fn input(&mut self, input: tui_textarea::Input) -> bool {
        self.text_area.input(input)
    }

    pub fn widget(&'a self) -> impl Widget + 'a {
        self.text_area.widget()
    }
}
