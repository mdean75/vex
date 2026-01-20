use crate::{
    learning::{Curriculum, Lesson, Task, ValidationResult, Validator},
    ui::Event,
    vim::{Buffer, Command, CommandParser, Executor},
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Menu,
    Lesson,
}

pub struct App {
    pub mode: AppMode,
    pub curriculum: Curriculum,
    pub current_lesson_id: Option<usize>,
    pub current_task_index: usize,
    pub task_buffer: Buffer,
    pub command_parser: CommandParser,
    pub validator: Validator,
    pub executed_commands: Vec<Command>,
    pub input_buffer: String,
    pub feedback_message: String,
    pub show_hint: bool,
    pub hint_index: usize,
    pub running: bool,
    pub waiting_for_next_task: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            curriculum: Curriculum::new(),
            current_lesson_id: None,
            current_task_index: 0,
            task_buffer: Buffer::new(""),
            command_parser: CommandParser::new(),
            validator: Validator::new(),
            executed_commands: Vec::new(),
            input_buffer: String::new(),
            feedback_message: String::new(),
            show_hint: false,
            hint_index: 0,
            running: true,
            waiting_for_next_task: false,
        }
    }

    pub fn current_lesson(&self) -> Option<&Lesson> {
        self.current_lesson_id
            .and_then(|id| self.curriculum.get_lesson(id))
    }

    pub fn current_task(&self) -> Option<&Task> {
        self.current_lesson()
            .and_then(|lesson| lesson.tasks.get(self.current_task_index))
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Quit => {
                self.running = false;
            }
            Event::Key(c) => {
                self.handle_key(c);
            }
            Event::SpecialKey(key) => {
                self.handle_special_key(key);
            }
            Event::Resize => {}
        }
    }

    fn handle_key(&mut self, c: char) {
        match self.mode {
            AppMode::Menu => self.handle_menu_key(c),
            AppMode::Lesson => self.handle_lesson_key(c),
        }
    }

    fn handle_special_key(&mut self, key: crate::ui::events::SpecialKey) {
        use crate::ui::events::SpecialKey;

        match key {
            SpecialKey::Escape => {
                if self.mode == AppMode::Lesson {
                    self.return_to_menu();
                }
            }
            SpecialKey::Enter => {}
            SpecialKey::Backspace => {
                if self.mode == AppMode::Lesson && self.command_parser.is_incomplete() {
                    self.command_parser.reset();
                    self.input_buffer.clear();
                    self.feedback_message = "Command cancelled.".to_string();
                }
            }
        }
    }

    fn handle_menu_key(&mut self, c: char) {
        match c {
            '1' | '2' | '3' | '4' | '5' | '6' => {
                let lesson_id = c.to_digit(10).unwrap() as usize;
                self.start_lesson(lesson_id);
            }
            'q' => {
                self.running = false;
            }
            _ => {}
        }
    }

    fn handle_lesson_key(&mut self, c: char) {
        match c {
            ' ' if self.waiting_for_next_task => {
                self.waiting_for_next_task = false;
                self.advance_to_next_task();
            }
            '?' if !self.command_parser.is_incomplete() => {
                self.show_next_hint();
            }
            'r' if !self.command_parser.is_incomplete() => {
                self.reset_current_task();
            }
            _ => {
                self.process_vim_command(c);
            }
        }
    }

    fn reset_current_task(&mut self) {
        self.reset_task_state();
        self.feedback_message = "Task reset. Try again!".to_string();
    }

    fn process_vim_command(&mut self, c: char) {
        self.input_buffer.push(c);

        let parse_result = self.command_parser.parse_keystroke(c);

        match parse_result {
            crate::vim::parser::ParseResult::Complete(command) => {
                if let Err(e) = Executor::execute(&mut self.task_buffer, command) {
                    self.feedback_message = format!("Error: {:?}", e);
                } else {
                    self.executed_commands.push(command);
                    self.check_task_completion();
                }
            }
            crate::vim::parser::ParseResult::Incomplete => {
                self.feedback_message = "Command incomplete...".to_string();
            }
            crate::vim::parser::ParseResult::Invalid(msg) => {
                self.feedback_message = format!("Invalid command: {}", msg);
                self.command_parser.reset();
                self.input_buffer.clear();
            }
        }
    }

    fn check_task_completion(&mut self) {
        if let Some(task) = self.current_task() {
            let result = self.validator.validate(
                &task.expected_commands,
                &self.executed_commands,
                self.task_buffer.cursor(),
                task.target_position,
            );

            match result {
                ValidationResult::Correct => {
                    self.show_hint = false;
                    self.hint_index = 0;

                    if self.current_task_index + 1 < self.current_lesson().unwrap().tasks.len() {
                        self.feedback_message = "Correct! Press Space to continue.".to_string();
                        self.waiting_for_next_task = true;
                    } else {
                        self.feedback_message = "Lesson completed! Press ESC to return to menu.".to_string();
                    }
                }
                ValidationResult::Incorrect { feedback } => {
                    self.feedback_message = feedback;
                }
                ValidationResult::WrongPosition { feedback } => {
                    self.feedback_message = feedback;
                }
            }
        }
    }

    fn advance_to_next_task(&mut self) {
        self.current_task_index += 1;
        self.reset_task_state();
        self.feedback_message = "Starting next task...".to_string();
    }

    fn show_next_hint(&mut self) {
        let hints_len = self.current_task().map(|t| t.hints.len()).unwrap_or(0);

        if hints_len == 0 {
            self.feedback_message = "No hints available for this task.".to_string();
            return;
        }

        if !self.show_hint {
            self.show_hint = true;
            self.hint_index = 0;
        } else if self.hint_index < hints_len - 1 {
            self.hint_index += 1;
        }
    }

    fn start_lesson(&mut self, lesson_id: usize) {
        if self.curriculum.get_lesson(lesson_id).is_some() {
            self.current_lesson_id = Some(lesson_id);
            self.current_task_index = 0;
            self.mode = AppMode::Lesson;
            self.reset_task_state();
            self.feedback_message = "Lesson started! Complete each task to progress.".to_string();
        }
    }

    fn return_to_menu(&mut self) {
        self.mode = AppMode::Menu;
        self.current_lesson_id = None;
        self.current_task_index = 0;
        self.reset_task_state();
        self.feedback_message = String::new();
    }

    fn reset_task_state(&mut self) {
        if let Some(task) = self.current_task() {
            self.task_buffer = task.initial_buffer.clone();
        } else {
            self.task_buffer = Buffer::new("");
        }

        self.executed_commands.clear();
        self.input_buffer.clear();
        self.command_parser.reset();
        self.show_hint = false;
        self.hint_index = 0;
        self.waiting_for_next_task = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
