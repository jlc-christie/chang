use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui_textarea::Input;
use anyhow::{Context, Result};
use base64::Engine;
use std::str;
use crate::widget::Signature;

#[derive(Clone)]
pub struct Chang<'a> {
    header: Paragraph<'a>,
    claims: Paragraph<'a>,
    signature: Signature<'a>,
    alg: jsonwebtoken::Algorithm,
    header_text: String,
    claims_text: String,
    signature_text: String,
}

impl Chang<'_> {
    pub fn new(jwt: impl Into<String>) -> Result<Self> {
        let jwt = jwt.into();
        let alg = Self::get_alg(&jwt).context("failed to parse alg from header of jwt")?;
        let (header_text, claims_text, signature_text) = Self::destructure_jwt(&jwt)
            .context("failed to destructure jwt")?;
        let header_text = Self::b64_to_json(header_text).context("failed to convert header b64 to json")?;
        let claims_text = Self::b64_to_json(claims_text).context("failed to convert claims b64 to json")?;

        let header = Paragraph::new(header_text.clone()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(251, 1, 91)))
                .title(" Header (^H) ")
        ).wrap(
            Wrap{ trim: false }
        );

        let claims = Paragraph::new(claims_text.clone()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(214, 58, 255)))
                .title(" Claims (^C) ")
        ).wrap(
            Wrap{ trim: false }
        );

        Ok(
            Chang {
                header,
                claims,
                signature: Signature::default(),
                alg,
                header_text,
                claims_text,
                signature_text: signature_text.to_string(),
            }
        )
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

    pub fn process_input(&mut self, input: Input) {
        self.signature.input(input);
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