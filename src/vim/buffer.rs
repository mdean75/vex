use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buffer {
    lines: Vec<String>,
    cursor: CursorPosition,
}

impl Buffer {
    pub fn new(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };

        Self {
            lines,
            cursor: CursorPosition::new(0, 0),
        }
    }

    pub fn from_lines(lines: Vec<String>) -> Self {
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };

        Self {
            lines,
            cursor: CursorPosition::new(0, 0),
        }
    }

    pub fn cursor(&self) -> CursorPosition {
        self.cursor
    }

    pub fn set_cursor(&mut self, position: CursorPosition) {
        self.cursor = position;
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    pub fn current_line(&self) -> &str {
        &self.lines[self.cursor.line]
    }

    pub fn current_line_len(&self) -> usize {
        self.current_line().len()
    }

    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    pub fn find_next_word_start(&self) -> CursorPosition {
        let line = self.current_line();
        let mut col = self.cursor.column;

        if col >= line.len() {
            return self.find_next_line_first_word();
        }

        let chars: Vec<char> = line.chars().collect();

        if col < chars.len() {
            let current_char = chars[col];

            if Self::is_word_char(current_char) {
                while col < chars.len() && Self::is_word_char(chars[col]) {
                    col += 1;
                }
            } else if !Self::is_whitespace(current_char) {
                while col < chars.len() && !Self::is_whitespace(chars[col]) && !Self::is_word_char(chars[col]) {
                    col += 1;
                }
            }

            while col < chars.len() && Self::is_whitespace(chars[col]) {
                col += 1;
            }
        }

        if col >= chars.len() {
            self.find_next_line_first_word()
        } else {
            CursorPosition::new(self.cursor.line, col)
        }
    }

    fn find_next_line_first_word(&self) -> CursorPosition {
        let mut line_idx = self.cursor.line + 1;
        while line_idx < self.lines.len() {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            for (col, &ch) in chars.iter().enumerate() {
                if !Self::is_whitespace(ch) {
                    return CursorPosition::new(line_idx, col);
                }
            }
            line_idx += 1;
        }

        CursorPosition::new(
            self.lines.len().saturating_sub(1),
            self.lines.last().map(|l| l.len().saturating_sub(1)).unwrap_or(0),
        )
    }

    pub fn find_prev_word_start(&self) -> CursorPosition {
        let line = self.current_line();
        let mut col = self.cursor.column;

        if col == 0 {
            return self.find_prev_line_last_word();
        }

        let chars: Vec<char> = line.chars().collect();
        col = col.saturating_sub(1);

        while col > 0 && Self::is_whitespace(chars[col]) {
            col -= 1;
        }

        if col == 0 && Self::is_whitespace(chars[col]) {
            return self.find_prev_line_last_word();
        }

        let current_is_word = Self::is_word_char(chars[col]);

        while col > 0 {
            let prev_char = chars[col - 1];
            if current_is_word {
                if !Self::is_word_char(prev_char) {
                    break;
                }
            } else {
                if Self::is_whitespace(prev_char) || Self::is_word_char(prev_char) {
                    break;
                }
            }
            col -= 1;
        }

        CursorPosition::new(self.cursor.line, col)
    }

    fn find_prev_line_last_word(&self) -> CursorPosition {
        if self.cursor.line == 0 {
            return CursorPosition::new(0, 0);
        }

        let mut line_idx = self.cursor.line - 1;
        loop {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            if !chars.is_empty() {
                let mut col = chars.len() - 1;

                while col > 0 && Self::is_whitespace(chars[col]) {
                    col -= 1;
                }

                if !Self::is_whitespace(chars[col]) {
                    let current_is_word = Self::is_word_char(chars[col]);
                    while col > 0 {
                        let prev_char = chars[col - 1];
                        if current_is_word {
                            if !Self::is_word_char(prev_char) {
                                break;
                            }
                        } else {
                            if Self::is_whitespace(prev_char) || Self::is_word_char(prev_char) {
                                break;
                            }
                        }
                        col -= 1;
                    }
                    return CursorPosition::new(line_idx, col);
                }
            }

            if line_idx == 0 {
                return CursorPosition::new(0, 0);
            }
            line_idx -= 1;
        }
    }

    pub fn find_word_end(&self) -> CursorPosition {
        let line = self.current_line();
        let mut col = self.cursor.column;
        let chars: Vec<char> = line.chars().collect();

        if col >= chars.len() {
            return self.find_next_line_word_end();
        }

        if col < chars.len() {
            col += 1;
        }

        while col < chars.len() && Self::is_whitespace(chars[col]) {
            col += 1;
        }

        if col >= chars.len() {
            return self.find_next_line_word_end();
        }

        let current_is_word = Self::is_word_char(chars[col]);

        while col < chars.len() {
            if current_is_word {
                if !Self::is_word_char(chars[col]) {
                    return CursorPosition::new(self.cursor.line, col - 1);
                }
            } else {
                if Self::is_whitespace(chars[col]) || Self::is_word_char(chars[col]) {
                    return CursorPosition::new(self.cursor.line, col - 1);
                }
            }
            col += 1;
        }

        CursorPosition::new(self.cursor.line, chars.len().saturating_sub(1))
    }

    fn find_next_line_word_end(&self) -> CursorPosition {
        let mut line_idx = self.cursor.line + 1;

        while line_idx < self.lines.len() {
            let line = &self.lines[line_idx];
            let chars: Vec<char> = line.chars().collect();

            let mut col = 0;
            while col < chars.len() && Self::is_whitespace(chars[col]) {
                col += 1;
            }

            if col < chars.len() {
                let current_is_word = Self::is_word_char(chars[col]);

                while col < chars.len() {
                    if current_is_word {
                        if !Self::is_word_char(chars[col]) {
                            return CursorPosition::new(line_idx, col - 1);
                        }
                    } else {
                        if Self::is_whitespace(chars[col]) || Self::is_word_char(chars[col]) {
                            return CursorPosition::new(line_idx, col - 1);
                        }
                    }
                    col += 1;
                }

                return CursorPosition::new(line_idx, chars.len().saturating_sub(1));
            }

            line_idx += 1;
        }

        CursorPosition::new(
            self.lines.len().saturating_sub(1),
            self.lines.last().map(|l| l.len().saturating_sub(1)).unwrap_or(0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new("Hello World");
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.cursor(), CursorPosition::new(0, 0));
    }

    #[test]
    fn test_word_boundaries() {
        let buffer = Buffer::new("foo.bar baz");
        buffer.cursor();

        let next_pos = buffer.find_next_word_start();
        assert_eq!(next_pos.column, 3);
    }

    #[test]
    fn test_multi_line_buffer() {
        let buffer = Buffer::new("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line_count(), 3);
    }
}
