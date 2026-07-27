pub mod canvas;
pub mod help_modal;
pub mod keypad;
pub mod top_bar;
pub mod variables_view;

use crate::app::App;
use keypad::Button;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) -> Vec<Button> {
    let area = frame.area();
    let mut buttons = Vec::new();

    // Determine layout mode based on terminal width and height
    let is_tiny = area.height < 26;
    let (is_big, is_medium) = if area.width >= 100 && area.height >= 24 {
        (true, false)
    } else if area.width >= 70 && area.height >= 18 {
        (false, true)
    } else {
        (false, false)
    };

    // Main Vertical Layout: Top Bar (3 lines), Body (Min 10 lines), Footer (1 line)
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .split(area);

    // Render Top Bar
    top_bar::render(frame, chunks[0], app);

    // Render Main Body
    if is_tiny {
        // Tiny Mode: Expression and result only, no keypad
        let body_split = Layout::vertical([
            Constraint::Min(4),
            Constraint::Length(3),
        ])
        .split(chunks[1]);

        canvas::render(frame, body_split[0], app);
        render_result_box(frame, body_split[1], app);
    } else if is_big {
        // Big Mode: Expression Canvas fills all remaining vertical space, Keypads get fixed 15-line height
        let body_split = Layout::vertical([
            Constraint::Min(6),      // Canvas & Result: Fills all remaining space!
            Constraint::Length(15),  // Keypads & Variables: 5 rows × 3 lines = comfortably clickable
        ])
        .split(chunks[1]);

        // Top Half Horizontal Split: Canvas (65%), Result (35%)
        let top_chunks = Layout::horizontal([
            Constraint::Percentage(65),
            Constraint::Percentage(35),
        ])
        .split(body_split[0]);

        canvas::render(frame, top_chunks[0], app);
        render_result_box(frame, top_chunks[1], app);

        // Bottom Half Horizontal Split: Basic Keypad (35%), Advanced Keypad (40%), Variables Table (25%)
        let bottom_chunks = Layout::horizontal([
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
        ])
        .split(body_split[1]);

        keypad::render_basic_keypad(frame, bottom_chunks[0], &mut buttons);
        keypad::render_advanced_keypad(frame, bottom_chunks[1], &mut buttons);
        variables_view::render(frame, bottom_chunks[2], app);
    } else if is_medium {
        // Medium Mode
        let body_split = Layout::vertical([
            Constraint::Min(6),
            Constraint::Length(15),  // 5 rows × 3 lines each = comfortably clickable buttons
        ])
        .split(chunks[1]);

        let top_chunks = Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(body_split[0]);

        canvas::render(frame, top_chunks[0], app);
        render_result_box(frame, top_chunks[1], app);

        let bottom_chunks = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(body_split[1]);

        keypad::render_basic_keypad(frame, bottom_chunks[0], &mut buttons);
        keypad::render_advanced_keypad(frame, bottom_chunks[1], &mut buttons);
    } else {
        // Small Mode: Compact stack with the result shown below the expression canvas
        let body_split = Layout::vertical([
            Constraint::Min(4),
            Constraint::Length(3),
            Constraint::Length(15), // 5 rows × 3 lines each
        ])
        .split(chunks[1]);

        canvas::render(frame, body_split[0], app);
        render_result_box(frame, body_split[1], app);
        keypad::render_basic_keypad(frame, body_split[2], &mut buttons);
    }

    // Footer / Error Bar
    render_footer(frame, chunks[2], app);

    // Help Modal Overlay
    if app.show_help {
        help_modal::render(frame, area);
    }

    buttons
}

fn render_result_box(frame: &mut Frame, area: Rect, app: &App) {
    let (text, color) = if let Some(ref err) = app.error_message {
        (format!("Error: {}", err), Color::Red)
    } else if app.result.is_empty() {
        ("=".to_string(), Color::DarkGray)
    } else {
        (format!("= {}", app.result), Color::Green)
    };

    let paragraph = Paragraph::new(Line::from(Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )))
    .block(
        Block::bordered()
            .title(" Result ")
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color)),
    );

    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let footer_text = if let Some(ref err) = app.error_message {
        Line::from(Span::styled(format!(" Error: {}", err), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
    } else {
        Line::from(Span::styled(" Ready ", Style::default().fg(Color::DarkGray)))
    };

    let paragraph = Paragraph::new(footer_text);
    frame.render_widget(paragraph, area);
}
