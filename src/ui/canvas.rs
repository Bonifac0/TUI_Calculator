use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let rendered = app.render_expression();
    let content_width = rendered
        .cells
        .iter()
        .map(|row| row.len())
        .max()
        .unwrap_or(1)
        .max(rendered.cursor_col + 1);
    let content_height = rendered.cells.len().max(1);

    let inner_width = area.width.saturating_sub(2) as usize;
    let inner_height = area.height.saturating_sub(2) as usize;

    if rendered.cursor_col < app.scroll_x {
        app.scroll_x = rendered.cursor_col;
    } else if inner_width > 0 && rendered.cursor_col >= app.scroll_x + inner_width {
        app.scroll_x = rendered.cursor_col.saturating_sub(inner_width - 1);
    }

    if rendered.cursor_row < app.scroll_y {
        app.scroll_y = rendered.cursor_row;
    } else if inner_height > 0 && rendered.cursor_row >= app.scroll_y + inner_height {
        app.scroll_y = rendered.cursor_row.saturating_sub(inner_height - 1);
    }

    let max_scroll_x = content_width.saturating_sub(inner_width.max(1));
    let max_scroll_y = content_height.saturating_sub(inner_height.max(1));
    app.scroll_x = app.scroll_x.min(max_scroll_x);
    app.scroll_y = app.scroll_y.min(max_scroll_y);

    let has_left = app.scroll_x > 0;
    let has_right = app.scroll_x + inner_width < content_width;
    let has_up = app.scroll_y > 0;
    let has_down = app.scroll_y + inner_height < content_height;
    let up_arrow = if has_up { "▲" } else { " " };
    let down_arrow = if has_down { "▼" } else { " " };
    let title_str = format!(" Expression Canvas ({} {}) ", up_arrow, down_arrow);

    let mut lines = Vec::new();
    for row_idx in 0..inner_height {
        let source_row = app.scroll_y + row_idx;
        let mut row_chars = vec![' '; inner_width];

        if source_row < rendered.cells.len() {
            let src = &rendered.cells[source_row];
            for (col_idx, ch) in row_chars.iter_mut().enumerate().take(inner_width) {
                let source_col = app.scroll_x + col_idx;
                if source_col < src.len() {
                    *ch = src[source_col];
                }
            }
        }

        let mut spans = Vec::new();
        for (col_idx, ch) in row_chars.into_iter().enumerate() {
            let source_col = app.scroll_x + col_idx;
            let source_row = app.scroll_y + row_idx;
            if source_row == rendered.cursor_row && source_col == rendered.cursor_col {
                spans.push(Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White)));
            }
        }
        lines.push(Line::from(spans));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            " ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::bordered()
            .title(title_str)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if has_left || has_right {
                Color::Yellow
            } else {
                Color::Blue
            })),
    );
    frame.render_widget(paragraph, area);
}
