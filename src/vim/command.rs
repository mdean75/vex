use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    Left,         // h
    Down,         // j
    Up,           // k
    Right,        // l
    WordForward,  // w
    WordBackward, // b
    WordEnd,      // e
    LineStart,    // 0
    LineEnd,      // $
    FileStart,    // gg
    FileEnd,      // G
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    Motion(Motion),
}

impl Command {
    pub fn as_motion(&self) -> Option<Motion> {
        match self {
            Command::Motion(m) => Some(*m),
        }
    }
}

impl Motion {
    pub fn to_key_str(&self) -> &'static str {
        match self {
            Motion::Left => "h",
            Motion::Down => "j",
            Motion::Up => "k",
            Motion::Right => "l",
            Motion::WordForward => "w",
            Motion::WordBackward => "b",
            Motion::WordEnd => "e",
            Motion::LineStart => "0",
            Motion::LineEnd => "$",
            Motion::FileStart => "gg",
            Motion::FileEnd => "G",
        }
    }
}
