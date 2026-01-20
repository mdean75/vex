use super::{Buffer, Command, CursorPosition, Motion};

#[derive(Debug)]
pub enum VimError {
    InvalidMotion(String),
}

pub struct Executor;

impl Executor {
    pub fn execute(buffer: &mut Buffer, command: Command) -> Result<(), VimError> {
        match command {
            Command::Motion(motion) => Self::execute_motion(buffer, motion),
        }
    }

    fn execute_motion(buffer: &mut Buffer, motion: Motion) -> Result<(), VimError> {
        let new_position = match motion {
            Motion::Left => Self::move_left(buffer),
            Motion::Down => Self::move_down(buffer),
            Motion::Up => Self::move_up(buffer),
            Motion::Right => Self::move_right(buffer),
            Motion::WordForward => buffer.find_next_word_start(),
            Motion::WordBackward => buffer.find_prev_word_start(),
            Motion::WordEnd => buffer.find_word_end(),
            Motion::LineStart => CursorPosition::new(buffer.cursor().line, 0),
            Motion::LineEnd => {
                let line_len = buffer.current_line_len();
                let col = if line_len > 0 { line_len - 1 } else { 0 };
                CursorPosition::new(buffer.cursor().line, col)
            }
            Motion::FileStart => CursorPosition::new(0, 0),
            Motion::FileEnd => {
                let last_line = buffer.line_count().saturating_sub(1);
                CursorPosition::new(last_line, 0)
            }
        };

        buffer.set_cursor(new_position);
        Ok(())
    }

    fn move_left(buffer: &Buffer) -> CursorPosition {
        let cursor = buffer.cursor();
        if cursor.column > 0 {
            CursorPosition::new(cursor.line, cursor.column - 1)
        } else {
            cursor
        }
    }

    fn move_right(buffer: &Buffer) -> CursorPosition {
        let cursor = buffer.cursor();
        let line_len = buffer.current_line_len();

        if line_len == 0 {
            return cursor;
        }

        let max_col = line_len - 1;
        if cursor.column < max_col {
            CursorPosition::new(cursor.line, cursor.column + 1)
        } else {
            cursor
        }
    }

    fn move_up(buffer: &Buffer) -> CursorPosition {
        let cursor = buffer.cursor();
        if cursor.line > 0 {
            let new_line = cursor.line - 1;
            let line_len = buffer.get_line(new_line).map(|l| l.len()).unwrap_or(0);

            let max_col = if line_len > 0 { line_len - 1 } else { 0 };
            let new_col = cursor.column.min(max_col);

            CursorPosition::new(new_line, new_col)
        } else {
            cursor
        }
    }

    fn move_down(buffer: &Buffer) -> CursorPosition {
        let cursor = buffer.cursor();
        if cursor.line < buffer.line_count() - 1 {
            let new_line = cursor.line + 1;
            let line_len = buffer.get_line(new_line).map(|l| l.len()).unwrap_or(0);

            let max_col = if line_len > 0 { line_len - 1 } else { 0 };
            let new_col = cursor.column.min(max_col);

            CursorPosition::new(new_line, new_col)
        } else {
            cursor
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_movement() {
        let mut buffer = Buffer::new("Hello World");

        Executor::execute(&mut buffer, Command::Motion(Motion::Right)).unwrap();
        assert_eq!(buffer.cursor().column, 1);

        Executor::execute(&mut buffer, Command::Motion(Motion::Left)).unwrap();
        assert_eq!(buffer.cursor().column, 0);
    }

    #[test]
    fn test_line_boundaries() {
        let mut buffer = Buffer::new("Test");
        buffer.set_cursor(CursorPosition::new(0, 3));

        Executor::execute(&mut buffer, Command::Motion(Motion::Right)).unwrap();
        assert_eq!(buffer.cursor().column, 3);
    }

    #[test]
    fn test_word_movement() {
        let mut buffer = Buffer::new("The quick brown");

        Executor::execute(&mut buffer, Command::Motion(Motion::WordForward)).unwrap();
        assert_eq!(buffer.cursor().column, 4);

        Executor::execute(&mut buffer, Command::Motion(Motion::WordForward)).unwrap();
        assert_eq!(buffer.cursor().column, 10);
    }
}
