use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(char),
    SpecialKey(SpecialKey),
    Quit,
    Resize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialKey {
    Escape,
    Enter,
    Backspace,
}

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn poll_event(&self, timeout: Duration) -> std::io::Result<Option<Event>> {
        if event::poll(timeout)? {
            match event::read()? {
                event::Event::Key(key_event) => Ok(Some(self.handle_key_event(key_event))),
                event::Event::Resize(_, _) => Ok(Some(Event::Resize)),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn handle_key_event(&self, key: KeyEvent) -> Event {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') | KeyCode::Char('q') => return Event::Quit,
                _ => {}
            }
        }

        match key.code {
            KeyCode::Char(c) => Event::Key(c),
            KeyCode::Esc => Event::SpecialKey(SpecialKey::Escape),
            KeyCode::Enter => Event::SpecialKey(SpecialKey::Enter),
            KeyCode::Backspace => Event::SpecialKey(SpecialKey::Backspace),
            _ => Event::SpecialKey(SpecialKey::Escape),
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
