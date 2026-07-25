use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mut items = Vec::new();

    // Render ans first
    let ans_val = app.variables.get_ans();
    items.push(ListItem::new(Line::from(vec![
        Span::styled("ans ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("= "),
        Span::styled(ans_val.to_formatted_string(app.precision), Style::default().fg(Color::Green)),
    ])));

    // Render A through F
    for c in 'A'..='F' {
        let name = c.to_string();
        let val = app.variables.get(&name).cloned().unwrap_or(crate::eval::Value::Scalar(0.0));
        items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("{}   ", name), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("= "),
            Span::styled(val.to_formatted_string(app.precision), Style::default().fg(Color::White)),
        ])));
    }

    let list = List::new(items).block(
        Block::bordered()
            .title(" Variables (A-F & ans) ")
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );

    frame.render_widget(list, area);
}
