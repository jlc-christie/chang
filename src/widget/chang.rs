use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use tui_textarea::{Input, TextArea};
use anyhow::{Context, Result};
use base64::Engine;
use std::str;
use crate::widget::Signature;

#[derive(Clone)]
pub enum FocusArea {
    Header,
    Claims,
    Signature
}

#[derive(Clone)]
pub struct Chang<'a> {
    header: TextArea<'a>,
    claims: TextArea<'a>,
    signature: Signature<'a>,
    alg: jsonwebtoken::Algorithm,
    header_text: String,
    claims_text: String,
    signature_text: String,
    focus_area: FocusArea
}

impl Chang<'_> {
    pub fn new(jwt: impl Into<String>) -> Result<Self> {
        let jwt = jwt.into();
        let alg = Self::get_alg(&jwt).context("failed to parse alg from header of jwt")?;
        let (header_text, claims_text, signature_text) = Self::destructure_jwt(&jwt)
            .context("failed to destructure jwt")?;
        let header_text = Self::b64_to_json(header_text).context("failed to convert header b64 to json")?;
        let claims_text = Self::b64_to_json(claims_text).context("failed to convert claims b64 to json")?;

        let mut header = TextArea::new(
            header_text.split('\n').map(|s| s.to_string()).collect()
        );
        header.set_line_number_style(Style::default().fg(Color::Rgb(251, 1, 91)));
        header.set_block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(251, 1, 91)))
            .title(" Header (^h) "));

        let mut claims = TextArea::new(
            claims_text.split('\n').map(|s| s.to_string()).collect()
        );
        claims.set_line_number_style(Style::default().fg(Color::Rgb(214, 58, 255)));
        claims.set_block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(214, 58, 255)))
            .title(" Claims (^b) "));

        Ok(
            Chang {
                header,
                claims,
                signature: Signature::default(),
                alg,
                header_text,
                claims_text,
                signature_text: signature_text.to_string(),
                focus_area: FocusArea::Signature
            }
        )
    }

    pub fn focus_area(&mut self, area: FocusArea) {
        self.focus_area = area
    }

    fn get_alg(jwt: &str) -> Result<jsonwebtoken::Algorithm> {
        let header = jsonwebtoken::decode_header(jwt)
            .context("failed to decode header")?;

        Ok(header.alg)
    }

    fn destructure_jwt(jwt: &str) -> Result<(&str, &str, &str)> {
        let parts = jwt.split('.').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("jwt is malformed, must have 3 parts"));
        }

        Ok((parts[0], parts[1], parts[2]))
    }

    fn b64_to_json(b64: &str) -> Result<String> {
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(b64).context("failed to decode b64")?;
        let text = str::from_utf8(bytes.as_slice()).context("failed to convert bytes to utf8 str")?;
        let json: serde_json::Value = serde_json::from_str(text).context("failed to convert text to json")?;
        let text = serde_json::to_string_pretty(&json).context("failed to pretty print")?;

        Ok(text)
    }

    pub fn process_input(&mut self, input: Input) -> bool {
        match self.focus_area {
            FocusArea::Header => self.header.input(input),
            FocusArea::Claims => self.claims.input(input),
            FocusArea::Signature => self.signature.input(input),
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

        self.header.widget().render(chunks[0], buf);
        self.claims.widget().render(chunks[1], buf);
        self.signature.widget().render(chunks[2], buf);
    }
}