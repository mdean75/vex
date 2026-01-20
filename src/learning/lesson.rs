use crate::vim::{Buffer, Command, CursorPosition};

#[derive(Clone)]
pub struct Lesson {
    pub id: usize,
    pub title: String,
    pub explanation: Vec<String>,
    pub commands: Vec<Command>,
    pub tasks: Vec<Task>,
}

#[derive(Clone)]
pub struct Task {
    pub description: String,
    pub initial_buffer: Buffer,
    pub target_position: CursorPosition,
    pub expected_commands: Vec<Command>,
    pub hints: Vec<String>,
}

impl Lesson {
    pub fn new(
        id: usize,
        title: impl Into<String>,
        explanation: Vec<String>,
        commands: Vec<Command>,
        tasks: Vec<Task>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            explanation,
            commands,
            tasks,
        }
    }
}

impl Task {
    pub fn new(
        description: impl Into<String>,
        initial_buffer: Buffer,
        target_position: CursorPosition,
        expected_commands: Vec<Command>,
        hints: Vec<String>,
    ) -> Self {
        Self {
            description: description.into(),
            initial_buffer,
            target_position,
            expected_commands,
            hints,
        }
    }
}
