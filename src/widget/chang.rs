use ratatui::prelude::*;
use ratatui::widgets::{Widget};
use tui_textarea::{Input};
use anyhow::{Context, Result};
use base64::Engine;
use std::str;
use crate::widget::{Claims, Header, Signature};

#[derive(Clone)]
pub enum FocusArea {
    Header,
    Claims,
    Signature
}

#[derive(Clone)]
pub struct Chang<'a> {
    header: Header<'a>,
    claims: Claims<'a>,
    signature: Signature<'a>,
    focus_area: FocusArea
}

impl Chang<'_> {
    pub fn new(jwt: impl Into<String>) -> Result<Self> {
        let jwt = jwt.into();
        let (header_text, claims_text, _) = Self::destructure_jwt(&jwt)
            .context("failed to destructure jwt")?;
        let header_text = Self::b64_to_json(header_text)
            .context("failed to convert header b64 to json")?;
        let claims_text = Self::b64_to_json(claims_text)
            .context("failed to convert claims b64 to json")?;

        let mut header = Header::new(
            header_text.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
        ).context("failed to create header area")?;
        header.set_focused(false);

        let mut claims = Claims::new(
            claims_text.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
        ).context("failed to create claims area")?;
        claims.set_focused(false);

        let signature = Signature::new(
            jwt.clone()
        ).context("failed to create signature area")?;

        Ok(
            Chang {
                header,
                claims,
                signature,
                focus_area: FocusArea::Signature
            }
        )
    }

    pub fn focus_area(&mut self, area: FocusArea) {
        match self.focus_area {
            FocusArea::Header => self.header.set_focused(false),
            FocusArea::Claims => self.claims.set_focused(false),
            FocusArea::Signature => self.signature.set_focused(false),
        }

        match area {
            FocusArea::Header => self.header.set_focused(true),
            FocusArea::Claims => self.claims.set_focused(true),
            FocusArea::Signature => self.signature.set_focused(true),
        }

        self.focus_area = area

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

#[cfg(test)]
mod tests {
    use std::fs;
    use crossterm::event::{KeyCode, KeyEvent};
    use super::*;

    #[test]
    fn test_chang_signature_validation_es256() {
        let jwt = fs::read_to_string("examples/es256/jwt")
            .expect("failed to read ES256 JWT from file");
        let mut chang = Chang::new(jwt)
            .expect("failed to create change from ES256 JWT");
        assert!(!chang.signature.valid);
        let key = fs::read_to_string("examples/es256/pub")
            .expect("failed to read ES256 public key from file");
        let input: Vec<Input> = key.chars()
            .map(|c| Input::from(KeyEvent::from(KeyCode::Char(c))))
            .collect();
        for input in input {
            chang.process_input(input);
        }
        assert!(chang.signature.valid);
    }

    #[test]
    fn test_chang_signature_validation_hs256() {
        let jwt = fs::read_to_string("examples/hs256/jwt")
            .expect("failed to read HS256 JWT from file");
        let mut chang = Chang::new(jwt)
            .expect("failed to create change from HS256 JWT");
        assert!(!chang.signature.valid);
        let key = fs::read_to_string("examples/hs256/key")
            .expect("failed to read HS256 key from file");
        let input: Vec<Input> = key.chars()
            .map(|c| Input::from(KeyEvent::from(KeyCode::Char(c))))
            .collect();
        for input in input {
            chang.process_input(input);
        }
        assert!(chang.signature.valid);
    }

    #[test]
    fn test_chang_signature_validation_ps256() {
        let jwt = fs::read_to_string("examples/ps256/jwt")
            .expect("failed to read PS256 JWT from file");
        let mut chang = Chang::new(jwt)
            .expect("failed to create change from PS256 JWT");
        assert!(!chang.signature.valid);
        let key = fs::read_to_string("examples/ps256/pub")
            .expect("failed to read PS256 public key from file");
        let input: Vec<Input> = key.chars()
            .map(|c| Input::from(KeyEvent::from(KeyCode::Char(c))))
            .collect();
        for input in input {
            chang.process_input(input);
        }
        assert!(chang.signature.valid);
    }

    #[test]
    fn test_chang_signature_validation_rs256() {
        let jwt = fs::read_to_string("examples/rs256/jwt")
            .expect("failed to read RS256 JWT from file");
        let mut chang = Chang::new(jwt)
            .expect("failed to create change from RS256 JWT");
        assert!(!chang.signature.valid);
        let key = fs::read_to_string("examples/rs256/pub")
            .expect("failed to read RS256 public key from file");
        let input: Vec<Input> = key.chars()
            .map(|c| Input::from(KeyEvent::from(KeyCode::Char(c))))
            .collect();
        for input in input {
            chang.process_input(input);
        }
        assert!(chang.signature.valid);
    }
}