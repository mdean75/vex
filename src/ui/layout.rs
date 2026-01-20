use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct AppLayout {
    pub header: Rect,
    pub instruction: Rect,
    pub buffer: Rect,
    pub feedback: Rect,
    pub footer: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(12),
                Constraint::Length(5),
                Constraint::Length(2),
            ])
            .split(area);

        Self {
            header: chunks[0],
            instruction: chunks[1],
            buffer: chunks[2],
            feedback: chunks[3],
            footer: chunks[4],
        }
    }
}

pub fn draw_ui(frame: &mut Frame, app: &crate::app::App) {
    let layout = AppLayout::new(frame.area());

    crate::ui::components::render_header(frame, layout.header, app);
    crate::ui::components::render_instruction(frame, layout.instruction, app);
    crate::ui::components::render_buffer(frame, layout.buffer, app);
    crate::ui::components::render_feedback(frame, layout.feedback, app);
    crate::ui::components::render_footer(frame, layout.footer, app);
}
