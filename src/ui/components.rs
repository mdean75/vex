use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppMode};

pub fn render_ui(frame: &mut Frame, app: &App) {
    crate::ui::layout::draw_ui(frame, app);
}

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let mode_text = match app.mode {
        AppMode::Menu => "Menu",
        AppMode::Lesson => "Lesson",
    };

    let lesson_info = if let AppMode::Lesson = app.mode {
        if let Some(lesson) = app.current_lesson() {
            format!("[Lesson {}/6: {}]", lesson.id, lesson.title)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let header_text = format!("VEX - Vim Trainer  |  {}  {}", mode_text, lesson_info);

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, area);
}

pub fn render_instruction(frame: &mut Frame, area: Rect, app: &App) {
    let content = match app.mode {
        AppMode::Menu => {
            vec![
                Line::from(Span::styled(
                    "Welcome to VEX - Vim Movement Trainer",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("Select a lesson to begin:"),
                Line::from(""),
                Line::from("  1. Basic Movement (hjkl)"),
                Line::from("  2. Word Movement (wbe)"),
                Line::from("  3. Line Extremes (0$)"),
                Line::from("  4. Combining Movements"),
                Line::from("  5. Vertical Movement (gg, G)"),
                Line::from("  6. Complete Movement Practice"),
                Line::from(""),
                Line::from("Press 1-6 to select a lesson, or 'q' to quit."),
            ]
        }
        AppMode::Lesson => {
            if let Some(lesson) = app.current_lesson() {
                let mut lines = vec![Line::from(Span::styled(
                    &lesson.title,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))];

                for line in &lesson.explanation {
                    lines.push(Line::from(line.as_str()));
                }

                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "────────────────────────────────────────────────────────",
                    Style::default().fg(Color::DarkGray),
                )));
                lines.push(Line::from(""));

                if let Some(task) = app.current_task() {
                    lines.push(Line::from(Span::styled(
                        format!("Task {}/{}: {}", app.current_task_index + 1, lesson.tasks.len(), task.description),
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )));
                }

                lines
            } else {
                vec![Line::from("No lesson selected")]
            }
        }
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

pub fn render_buffer(frame: &mut Frame, area: Rect, app: &App) {
    if let AppMode::Lesson = app.mode {
        if let Some(_task) = app.current_task() {
            let buffer = &app.task_buffer;
            let cursor = buffer.cursor();

            let mut lines = Vec::new();

            for (line_idx, line_text) in buffer.lines().iter().enumerate() {
                if line_idx == cursor.line {
                    let mut spans = Vec::new();

                    for (col_idx, ch) in line_text.chars().enumerate() {
                        if col_idx == cursor.column {
                            spans.push(Span::styled(
                                ch.to_string(),
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ));
                        } else {
                            spans.push(Span::raw(ch.to_string()));
                        }
                    }

                    if cursor.column >= line_text.len() {
                        spans.push(Span::styled(
                            " ",
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ));
                    }

                    if buffer.line_count() > 1 {
                        let line_num = format!("{:2} │ ", line_idx + 1);
                        let mut line_spans = vec![Span::styled(
                            line_num,
                            Style::default().fg(Color::DarkGray),
                        )];
                        line_spans.extend(spans);
                        lines.push(Line::from(line_spans));
                    } else {
                        lines.push(Line::from(spans));
                    }
                } else {
                    if buffer.line_count() > 1 {
                        let line_num = format!("{:2} │ ", line_idx + 1);
                        lines.push(Line::from(vec![
                            Span::styled(line_num, Style::default().fg(Color::DarkGray)),
                            Span::raw(line_text.clone()),
                        ]));
                    } else {
                        lines.push(Line::from(line_text.clone()));
                    }
                }
            }

            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Buffer"))
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, area);
            return;
        }
    }

    let empty = Paragraph::new("No buffer to display")
        .block(Block::default().borders(Borders::ALL).title("Buffer"));
    frame.render_widget(empty, area);
}

pub fn render_feedback(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        Span::styled("Input: ", Style::default().fg(Color::Gray)),
        Span::styled(
            &app.input_buffer,
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    if !app.feedback_message.is_empty() {
        let style = if app.feedback_message.contains("Correct") || app.feedback_message.contains("completed") {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if app.feedback_message.contains("Incorrect") || app.feedback_message.contains("Wrong") {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Yellow)
        };

        lines.push(Line::from(Span::styled(&app.feedback_message, style)));
    }

    if app.show_hint {
        if let Some(task) = app.current_task() {
            let hint_index = app.hint_index.min(task.hints.len().saturating_sub(1));
            if hint_index < task.hints.len() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Hint: {}", task.hints[hint_index]),
                    Style::default().fg(Color::Cyan),
                )));
            }
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Feedback"))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

pub fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let footer_text = match app.mode {
        AppMode::Menu => "[1-6] Select Lesson  |  [q] Quit",
        AppMode::Lesson => "[r] Reset Task  |  [?] Hint  |  [ESC] Menu  |  [Ctrl+Q] Quit",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(footer, area);
}
