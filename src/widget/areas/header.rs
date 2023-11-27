use tui_textarea::TextArea;
use anyhow::{Result};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Widget};

#[derive(Clone)]
pub struct Header<'a> {
    text_area: TextArea<'a>,
    focused: bool,
}

impl<'a> Header<'a> {
    pub fn new(text: impl Into<Vec<String>>) -> Result<Self> {
        let mut text_area = TextArea::new(text.into());
        text_area.set_line_number_style(Style::default().fg(Color::Rgb(251, 1, 91)));
        text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(251, 1, 91)))
            .title(" Header (^h) "));

        Ok(
            Header {
                text_area,
                // TODO(jlc-christie): default to false with dimmed borders
                focused: true,
            }
        )
    }

    pub fn input(&mut self, input: tui_textarea::Input) -> bool {
        self.text_area.input(input)
    }

    // TODO(jlc-christie): change this so that header just implements Widget directly, can then also
    // use the focused state to render correct border without having to change it in set_focus
    pub fn widget(&'a self) -> impl Widget + 'a {
        self.text_area.widget()
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;

        // TODO(jlc-christie): why can't we use `?` on the optional below?
        let mut block = self.text_area.block().cloned().expect("failed to unwrap header text area block");
        if focused {
            block = block.not_dim();
        } else {
            block = block.dim();
        }
        self.text_area.set_block(block);
    }
}
