mod widget;

use std::io;
use anyhow::{Context, Result};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, Event, read},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};
use tui_textarea::{Input, Key};
use log::{error};
use clap::{Parser};
use widget::Chang;
use crate::widget::FocusArea;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    jwt: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut chang = Chang::new(args.jwt).context("failed to create chang from provided jwt")?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = event_loop(&mut terminal, &mut chang);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, chang: &mut Chang) -> Result<()> {
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
                    Input{ctrl: true, key: Key::Char('h'), ..} => chang.focus_area(FocusArea::Header),
                    Input{ctrl: true, key: Key::Char('b'), ..} => chang.focus_area(FocusArea::Claims),
                    Input{ctrl: true, key: Key::Char('d'), ..} => chang.focus_area(FocusArea::Signature),
                    input => {
                        if !chang.process_input(input.clone()) {
                            error!("failed to process input: {:?}", input)
                        }
                    },
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}
