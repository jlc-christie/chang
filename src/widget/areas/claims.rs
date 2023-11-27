use tui_textarea::TextArea;
use anyhow::Result;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

#[derive(Clone)]
pub struct Claims<'a> {
    text_area: TextArea<'a>
}

impl<'a> Claims<'a> {
    pub fn new(text: impl Into<Vec<String>>) -> Result<Self> {
        let mut text_area = TextArea::new(text.into());
        // 167, 136, 175 @ 20% Luminance
        text_area.set_line_number_style(Style::default().fg(Color::Rgb(214, 58, 255)));
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(214, 58, 255)))
            .title(" Claims (^b) "));
        Ok(
            Claims{
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