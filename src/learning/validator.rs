use crate::vim::{Command, CursorPosition};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Correct,
    Incorrect { feedback: String },
    WrongPosition { feedback: String },
}

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(
        &self,
        expected: &[Command],
        actual: &[Command],
        final_position: CursorPosition,
        target_position: CursorPosition,
    ) -> ValidationResult {
        if actual.len() != expected.len() {
            return ValidationResult::Incorrect {
                feedback: format!(
                    "Expected {} commands, but got {}. Try using only the commands taught in this lesson.",
                    expected.len(),
                    actual.len()
                ),
            };
        }

        for (i, (exp, act)) in expected.iter().zip(actual.iter()).enumerate() {
            if exp != act {
                let exp_motion = exp.as_motion().unwrap();
                return ValidationResult::Incorrect {
                    feedback: format!(
                        "Command {} was incorrect. Expected '{}', got '{}'.",
                        i + 1,
                        exp_motion.to_key_str(),
                        act.as_motion().unwrap().to_key_str()
                    ),
                };
            }
        }

        if final_position != target_position {
            return ValidationResult::WrongPosition {
                feedback: format!(
                    "Commands were correct, but cursor is at position ({}, {}). Target is ({}, {}).",
                    final_position.line, final_position.column,
                    target_position.line, target_position.column
                ),
            };
        }

        ValidationResult::Correct
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vim::Motion;

    #[test]
    fn test_correct_validation() {
        let validator = Validator::new();
        let expected = vec![Command::Motion(Motion::Right), Command::Motion(Motion::Right)];
        let actual = vec![Command::Motion(Motion::Right), Command::Motion(Motion::Right)];

        let result = validator.validate(
            &expected,
            &actual,
            CursorPosition::new(0, 2),
            CursorPosition::new(0, 2),
        );

        assert_eq!(result, ValidationResult::Correct);
    }

    #[test]
    fn test_wrong_command() {
        let validator = Validator::new();
        let expected = vec![Command::Motion(Motion::Right)];
        let actual = vec![Command::Motion(Motion::Left)];

        let result = validator.validate(
            &expected,
            &actual,
            CursorPosition::new(0, 0),
            CursorPosition::new(0, 1),
        );

        match result {
            ValidationResult::Incorrect { .. } => (),
            _ => panic!("Expected incorrect result"),
        }
    }
}
