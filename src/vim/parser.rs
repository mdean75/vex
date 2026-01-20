use super::{Command, Motion};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult {
    Complete(Command),
    Incomplete,
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Initial,
    GotG,
}

pub struct CommandParser {
    state: ParserState,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Initial,
        }
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Initial;
    }

    pub fn parse_keystroke(&mut self, key: char) -> ParseResult {
        match self.state {
            ParserState::Initial => self.parse_initial(key),
            ParserState::GotG => self.parse_after_g(key),
        }
    }

    fn parse_initial(&mut self, key: char) -> ParseResult {
        match key {
            'h' => ParseResult::Complete(Command::Motion(Motion::Left)),
            'j' => ParseResult::Complete(Command::Motion(Motion::Down)),
            'k' => ParseResult::Complete(Command::Motion(Motion::Up)),
            'l' => ParseResult::Complete(Command::Motion(Motion::Right)),
            'w' => ParseResult::Complete(Command::Motion(Motion::WordForward)),
            'b' => ParseResult::Complete(Command::Motion(Motion::WordBackward)),
            'e' => ParseResult::Complete(Command::Motion(Motion::WordEnd)),
            '0' => ParseResult::Complete(Command::Motion(Motion::LineStart)),
            '$' => ParseResult::Complete(Command::Motion(Motion::LineEnd)),
            'G' => ParseResult::Complete(Command::Motion(Motion::FileEnd)),
            'g' => {
                self.state = ParserState::GotG;
                ParseResult::Incomplete
            }
            _ => ParseResult::Invalid(format!("Unknown key: '{}'", key)),
        }
    }

    fn parse_after_g(&mut self, key: char) -> ParseResult {
        self.state = ParserState::Initial;

        match key {
            'g' => ParseResult::Complete(Command::Motion(Motion::FileStart)),
            _ => ParseResult::Invalid(format!("Expected 'g' after 'g', got '{}'", key)),
        }
    }

    pub fn is_incomplete(&self) -> bool {
        self.state != ParserState::Initial
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_motions() {
        let mut parser = CommandParser::new();

        assert_eq!(
            parser.parse_keystroke('h'),
            ParseResult::Complete(Command::Motion(Motion::Left))
        );
        assert_eq!(
            parser.parse_keystroke('j'),
            ParseResult::Complete(Command::Motion(Motion::Down))
        );
    }

    #[test]
    fn test_gg_command() {
        let mut parser = CommandParser::new();

        assert_eq!(parser.parse_keystroke('g'), ParseResult::Incomplete);
        assert_eq!(
            parser.parse_keystroke('g'),
            ParseResult::Complete(Command::Motion(Motion::FileStart))
        );
    }

    #[test]
    fn test_invalid_after_g() {
        let mut parser = CommandParser::new();

        assert_eq!(parser.parse_keystroke('g'), ParseResult::Incomplete);
        match parser.parse_keystroke('x') {
            ParseResult::Invalid(_) => (),
            _ => panic!("Expected invalid result"),
        }
    }
}
