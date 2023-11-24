mod widget;

use std::io;
use anyhow::{Result};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, Event, read},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, Terminal
};
use tui_textarea::{Input, Key};
use widget::Chang;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = event_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut chang = Chang::default();

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
                    input => chang.process_input(input),
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}
