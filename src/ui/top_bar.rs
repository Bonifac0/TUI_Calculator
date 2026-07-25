use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mode_str = format!("{:?}", app.angle_unit).to_uppercase();

    let title_line = Line::from(vec![
        Span::styled(" TUI Calculator ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" │ "),
        Span::styled(format!(" [{}] ", mode_str), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" │ "),
        Span::styled("Press '?' or F1 for Help", Style::default().fg(Color::Green)),
        Span::raw(" │ "),
        Span::styled("Press 'q' / Esc to Quit", Style::default().fg(Color::Magenta)),
    ]);

    let log_line = Line::from(vec![
        Span::styled(" Log: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&app.debug_log, Style::default().fg(Color::Gray)),
    ]);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(vec![title_line, log_line]).block(block);
    frame.render_widget(paragraph, area);
}
