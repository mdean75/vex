use super::{Lesson, Task};
use crate::vim::{Buffer, Command, CursorPosition, Motion};

pub struct Curriculum {
    lessons: Vec<Lesson>,
}

impl Curriculum {
    pub fn new() -> Self {
        Self {
            lessons: vec![
                Self::lesson_1_basic_movement(),
                Self::lesson_2_word_movement(),
                Self::lesson_3_line_extremes(),
                Self::lesson_4_combining_movements(),
                Self::lesson_5_vertical_movement(),
                Self::lesson_6_complete_practice(),
            ],
        }
    }

    pub fn lessons(&self) -> &[Lesson] {
        &self.lessons
    }

    pub fn get_lesson(&self, id: usize) -> Option<&Lesson> {
        self.lessons.iter().find(|l| l.id == id)
    }

    fn lesson_1_basic_movement() -> Lesson {
        let explanation = vec![
            "Welcome to VEX! Let's start with the foundation of Vim movement.".to_string(),
            "".to_string(),
            "The 'h', 'j', 'k', 'l' keys move the cursor:".to_string(),
            "  h - move left".to_string(),
            "  j - move down".to_string(),
            "  k - move up".to_string(),
            "  l - move right".to_string(),
            "".to_string(),
            "These replace the arrow keys and keep your hands on the home row.".to_string(),
        ];

        let commands = vec![
            Command::Motion(Motion::Left),
            Command::Motion(Motion::Down),
            Command::Motion(Motion::Up),
            Command::Motion(Motion::Right),
        ];

        let buffer_text = "Line one\nLine two\nLine three\nLine four";

        let tasks = vec![
            Task::new(
                "Move the cursor right to the 'i' in 'Line' (press l)",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 1),
                vec![Command::Motion(Motion::Right)],
                vec![
                    "Use the 'l' key to move right.".to_string(),
                    "Press 'l' once.".to_string(),
                    "Answer: l".to_string(),
                ],
            ),
            Task::new(
                "Move the cursor down to 'L' in 'Line two' (press j)",
                Buffer::new(buffer_text),
                CursorPosition::new(1, 0),
                vec![Command::Motion(Motion::Down)],
                vec![
                    "Use the 'j' key to move down.".to_string(),
                    "Press 'j' once.".to_string(),
                    "Answer: j".to_string(),
                ],
            ),
            Task::new(
                "Move right 3 times to reach 'e' in 'Line one' (press lll)",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 3),
                vec![
                    Command::Motion(Motion::Right),
                    Command::Motion(Motion::Right),
                    Command::Motion(Motion::Right),
                ],
                vec![
                    "Press 'l' three times.".to_string(),
                    "Each 'l' moves right by one character.".to_string(),
                    "Answer: lll".to_string(),
                ],
            ),
            Task::new(
                "From 'o' in 'one', move down to 't' in 'two' (press j)",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 5));
                    buf
                },
                CursorPosition::new(1, 5),
                vec![Command::Motion(Motion::Down)],
                vec![
                    "You need to move down from line one to line two.".to_string(),
                    "Use 'j' to move down one line.".to_string(),
                    "Answer: j".to_string(),
                ],
            ),
            Task::new(
                "From 'n' in 'Line', move left to 'i' (press h)",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 2));
                    buf
                },
                CursorPosition::new(0, 1),
                vec![Command::Motion(Motion::Left)],
                vec![
                    "Use the 'h' key to move left.".to_string(),
                    "Press 'h' once.".to_string(),
                    "Answer: h".to_string(),
                ],
            ),
            Task::new(
                "From 'L' in 'Line two', move up to 'L' in 'Line one' (press k)",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(1, 0));
                    buf
                },
                CursorPosition::new(0, 0),
                vec![Command::Motion(Motion::Up)],
                vec![
                    "Use the 'k' key to move up.".to_string(),
                    "Press 'k' once.".to_string(),
                    "Answer: k".to_string(),
                ],
            ),
        ];

        Lesson::new(1, "Basic Movement (hjkl)", explanation, commands, tasks)
    }

    fn lesson_2_word_movement() -> Lesson {
        let explanation = vec![
            "Word-based movement is much faster than moving character by character.".to_string(),
            "".to_string(),
            "Word movement commands:".to_string(),
            "  w - move forward to the start of the next word".to_string(),
            "  b - move backward to the start of the previous word".to_string(),
            "  e - move forward to the end of the current/next word".to_string(),
            "".to_string(),
            "In Vim, punctuation counts as separate words!".to_string(),
        ];

        let commands = vec![
            Command::Motion(Motion::WordForward),
            Command::Motion(Motion::WordBackward),
            Command::Motion(Motion::WordEnd),
        ];

        let buffer_text = "The quick brown fox jumps over the lazy dog";

        let tasks = vec![
            Task::new(
                "Move to the start of 'quick'",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 4),
                vec![Command::Motion(Motion::WordForward)],
                vec![
                    "Use 'w' to jump to the next word.".to_string(),
                    "Press 'w' once to move from 'The' to 'quick'.".to_string(),
                    "Answer: w".to_string(),
                ],
            ),
            Task::new(
                "Move to the start of 'fox'",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 16),
                vec![
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                ],
                vec![
                    "You need to move forward three words.".to_string(),
                    "Press 'w' three times: The→quick→brown→fox".to_string(),
                    "Answer: www".to_string(),
                ],
            ),
            Task::new(
                "Move to the end of 'brown'",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 10));
                    buf
                },
                CursorPosition::new(0, 14),
                vec![Command::Motion(Motion::WordEnd)],
                vec![
                    "Use 'e' to jump to the end of a word.".to_string(),
                    "Press 'e' once to move to the end of 'brown'.".to_string(),
                    "Answer: e".to_string(),
                ],
            ),
            Task::new(
                "Go back to 'quick' from 'fox'",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 16));
                    buf
                },
                CursorPosition::new(0, 4),
                vec![
                    Command::Motion(Motion::WordBackward),
                    Command::Motion(Motion::WordBackward),
                ],
                vec![
                    "Use 'b' to move backward by words.".to_string(),
                    "Press 'b' twice: fox→brown→quick".to_string(),
                    "Answer: bb".to_string(),
                ],
            ),
        ];

        Lesson::new(2, "Word Movement (wbe)", explanation, commands, tasks)
    }

    fn lesson_3_line_extremes() -> Lesson {
        let explanation = vec![
            "Jumping to the beginning or end of a line is a common operation.".to_string(),
            "".to_string(),
            "Line movement commands:".to_string(),
            "  0 - jump to the start of the line (column 0)".to_string(),
            "  $ - jump to the end of the line".to_string(),
            "".to_string(),
            "These are much faster than holding 'h' or 'l'!".to_string(),
        ];

        let commands = vec![
            Command::Motion(Motion::LineStart),
            Command::Motion(Motion::LineEnd),
        ];

        let buffer_text = "The quick brown fox jumps over the lazy dog";

        let tasks = vec![
            Task::new(
                "Jump to the beginning of the line",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 20));
                    buf
                },
                CursorPosition::new(0, 0),
                vec![Command::Motion(Motion::LineStart)],
                vec![
                    "Use '0' to jump to the start of the line.".to_string(),
                    "Press '0' (zero) once.".to_string(),
                    "Answer: 0".to_string(),
                ],
            ),
            Task::new(
                "Jump to the end of the line",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 42),
                vec![Command::Motion(Motion::LineEnd)],
                vec![
                    "Use '$' to jump to the end of the line.".to_string(),
                    "Press '$' once.".to_string(),
                    "Answer: $".to_string(),
                ],
            ),
            Task::new(
                "Go to the end, then back to the start",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 15));
                    buf
                },
                CursorPosition::new(0, 0),
                vec![
                    Command::Motion(Motion::LineEnd),
                    Command::Motion(Motion::LineStart),
                ],
                vec![
                    "First jump to the end with '$', then to the start with '0'.".to_string(),
                    "Press '$' then '0'.".to_string(),
                    "Answer: $0".to_string(),
                ],
            ),
        ];

        Lesson::new(3, "Line Extremes (0$)", explanation, commands, tasks)
    }

    fn lesson_4_combining_movements() -> Lesson {
        let explanation = vec![
            "Now let's practice combining different movement commands.".to_string(),
            "".to_string(),
            "You can chain movements to reach your target efficiently:".to_string(),
            "  - Use line jumps (0, $) to get to line boundaries".to_string(),
            "  - Use word jumps (w, b, e) to navigate between words".to_string(),
            "  - Use character moves (h, j, k, l) for fine adjustments".to_string(),
            "".to_string(),
            "Think about the most efficient path to your destination!".to_string(),
        ];

        let commands = vec![
            Command::Motion(Motion::Left),
            Command::Motion(Motion::Right),
            Command::Motion(Motion::WordForward),
            Command::Motion(Motion::WordBackward),
            Command::Motion(Motion::WordEnd),
            Command::Motion(Motion::LineStart),
            Command::Motion(Motion::LineEnd),
        ];

        let buffer_text = "The quick brown fox jumps over the lazy dog";

        let tasks = vec![
            Task::new(
                "Navigate from start to 'lazy' using word movement",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 35),
                vec![
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                ],
                vec![
                    "Count the words: The quick brown fox jumps over the lazy".to_string(),
                    "That's 7 words forward. Use 'w' seven times.".to_string(),
                    "Answer: wwwwwww".to_string(),
                ],
            ),
            Task::new(
                "Jump to end, then back 3 words",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 31),
                vec![
                    Command::Motion(Motion::LineEnd),
                    Command::Motion(Motion::WordBackward),
                    Command::Motion(Motion::WordBackward),
                    Command::Motion(Motion::WordBackward),
                ],
                vec![
                    "First use '$' to reach the end.".to_string(),
                    "Then use 'b' three times to go back.".to_string(),
                    "Answer: $bbb".to_string(),
                ],
            ),
            Task::new(
                "From 'jumps' to 'brown' using line and word movement",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(0, 20));
                    buf
                },
                CursorPosition::new(0, 10),
                vec![
                    Command::Motion(Motion::LineStart),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                ],
                vec![
                    "Jump to start with '0', then forward to 'brown'.".to_string(),
                    "Use '0' then 'w' twice.".to_string(),
                    "Answer: 0ww".to_string(),
                ],
            ),
        ];

        Lesson::new(
            4,
            "Combining Movements",
            explanation,
            commands,
            tasks,
        )
    }

    fn lesson_5_vertical_movement() -> Lesson {
        let explanation = vec![
            "For navigating files, you need vertical movement commands.".to_string(),
            "".to_string(),
            "File movement commands:".to_string(),
            "  gg - jump to the first line of the file".to_string(),
            "  G  - jump to the last line of the file".to_string(),
            "".to_string(),
            "These are essential for quickly moving through large files.".to_string(),
        ];

        let commands = vec![Command::Motion(Motion::FileStart), Command::Motion(Motion::FileEnd)];

        let buffer_text = "First line of text\n\
                          Second line here\n\
                          Third line content\n\
                          Fourth line follows\n\
                          Fifth line present\n\
                          Sixth line exists\n\
                          Seventh and final line";

        let tasks = vec![
            Task::new(
                "Jump to the last line of the file",
                Buffer::new(buffer_text),
                CursorPosition::new(6, 0),
                vec![Command::Motion(Motion::FileEnd)],
                vec![
                    "Use 'G' (capital G) to jump to the last line.".to_string(),
                    "Press 'G' once.".to_string(),
                    "Answer: G".to_string(),
                ],
            ),
            Task::new(
                "Jump to the first line of the file",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(4, 0));
                    buf
                },
                CursorPosition::new(0, 0),
                vec![Command::Motion(Motion::FileStart)],
                vec![
                    "Use 'gg' to jump to the first line.".to_string(),
                    "Press 'g' twice quickly.".to_string(),
                    "Answer: gg".to_string(),
                ],
            ),
            Task::new(
                "Jump to the end and back to the start",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(3, 0));
                    buf
                },
                CursorPosition::new(0, 0),
                vec![
                    Command::Motion(Motion::FileEnd),
                    Command::Motion(Motion::FileStart),
                ],
                vec![
                    "First jump to the end with 'G', then back to start with 'gg'.".to_string(),
                    "Press 'G' then 'gg'.".to_string(),
                    "Answer: Ggg".to_string(),
                ],
            ),
        ];

        Lesson::new(5, "Vertical Movement (gg, G)", explanation, commands, tasks)
    }

    fn lesson_6_complete_practice() -> Lesson {
        let explanation = vec![
            "Time to practice everything you've learned!".to_string(),
            "".to_string(),
            "This lesson combines all movement commands:".to_string(),
            "  - Basic movement: h, j, k, l".to_string(),
            "  - Word movement: w, b, e".to_string(),
            "  - Line movement: 0, $".to_string(),
            "  - File movement: gg, G".to_string(),
            "".to_string(),
            "Think about the most efficient way to reach each target!".to_string(),
        ];

        let commands = vec![
            Command::Motion(Motion::Left),
            Command::Motion(Motion::Down),
            Command::Motion(Motion::Up),
            Command::Motion(Motion::Right),
            Command::Motion(Motion::WordForward),
            Command::Motion(Motion::WordBackward),
            Command::Motion(Motion::WordEnd),
            Command::Motion(Motion::LineStart),
            Command::Motion(Motion::LineEnd),
            Command::Motion(Motion::FileStart),
            Command::Motion(Motion::FileEnd),
        ];

        let buffer_text = "function calculate(x, y) {\n\
                          let result = x + y;\n\
                          return result;\n\
                          }\n\
                          \n\
                          function main() {\n\
                          let value = calculate(10, 20);\n\
                          console.log(value);\n\
                          }";

        let tasks = vec![
            Task::new(
                "Navigate to 'calculate' on the first line",
                Buffer::new(buffer_text),
                CursorPosition::new(0, 9),
                vec![Command::Motion(Motion::WordForward)],
                vec![
                    "From the start, move forward one word.".to_string(),
                    "Use 'w' once.".to_string(),
                    "Answer: w".to_string(),
                ],
            ),
            Task::new(
                "Jump to the last line, then to 'log'",
                Buffer::new(buffer_text),
                CursorPosition::new(7, 8),
                vec![
                    Command::Motion(Motion::FileEnd),
                    Command::Motion(Motion::Up),
                    Command::Motion(Motion::WordForward),
                    Command::Motion(Motion::WordForward),
                ],
                vec![
                    "First jump to the last line with 'G'.".to_string(),
                    "Then move up with 'k' and forward two words with 'w'.".to_string(),
                    "Answer: Gkww".to_string(),
                ],
            ),
            Task::new(
                "From line 3, go to end of 'result' on line 2",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(2, 7));
                    buf
                },
                CursorPosition::new(1, 9),
                vec![
                    Command::Motion(Motion::Up),
                    Command::Motion(Motion::WordEnd),
                ],
                vec![
                    "Move up one line, then to the end of the word.".to_string(),
                    "Use 'k' to move up, then 'e' to jump to word end.".to_string(),
                    "Answer: ke".to_string(),
                ],
            ),
            Task::new(
                "Navigate from 'function main()' to the opening brace of calculate",
                {
                    let mut buf = Buffer::new(buffer_text);
                    buf.set_cursor(CursorPosition::new(5, 0));
                    buf
                },
                CursorPosition::new(0, 25),
                vec![
                    Command::Motion(Motion::FileStart),
                    Command::Motion(Motion::LineEnd),
                ],
                vec![
                    "Jump to the first line, then to the end of that line.".to_string(),
                    "Use 'gg' then '$'.".to_string(),
                    "Answer: gg$".to_string(),
                ],
            ),
        ];

        Lesson::new(
            6,
            "Complete Movement Practice",
            explanation,
            commands,
            tasks,
        )
    }
}

impl Default for Curriculum {
    fn default() -> Self {
        Self::new()
    }
}
