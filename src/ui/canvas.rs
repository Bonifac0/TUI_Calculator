use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let input_str = &app.input;
    let cursor = app.cursor_pos;

    // Build rendered text line with cursor indicator
    let mut spans = Vec::new();

    if input_str.is_empty() {
        spans.push(Span::styled("│", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    } else {
        for (i, ch) in input_str.chars().enumerate() {
            if i == cursor {
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White)));
            }
        }
        if cursor >= input_str.len() {
            spans.push(Span::styled("│", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        }
    }

    // Determine 4-way scroll indicator arrows
    let has_left = app.scroll_x > 0;
    let has_right = input_str.len() > area.width as usize;
    let has_up = app.scroll_y > 0;
    let has_down = false; // Expandable for 2D multiline matrices

    let left_arrow = if has_left { "◀ " } else { "  " };
    let right_arrow = if has_right { " ▶" } else { "  " };
    let up_arrow = if has_up { "▲" } else { " " };
    let down_arrow = if has_down { "▼" } else { " " };

    let title_str = format!(" Expression Canvas ({} {}) ", up_arrow, down_arrow);

    let display_line = Line::from(spans);

    let block = Block::bordered()
        .title(title_str)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));

    let content_line = Line::from(vec![
        Span::styled(left_arrow, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Expression "),
        Span::styled(right_arrow, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);

    let mut lines = vec![display_line];
    if has_left || has_right {
        lines.insert(0, content_line);
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
