use crate::input::AppAction;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct Button {
    pub label: &'static str,
    pub action: AppAction,
    pub area: Rect,
    pub color: Color,
}

pub fn render_basic_keypad(frame: &mut Frame, area: Rect, buttons_out: &mut Vec<Button>) {
    let rows_spec = [
        vec![("C", AppAction::ClearInput, Color::Red), ("AC", AppAction::AllClear, Color::Red), ("⌫", AppAction::Backspace, Color::Red), ("÷", AppAction::InsertChar('/'), Color::Yellow)],
        vec![("7", AppAction::InsertChar('7'), Color::White), ("8", AppAction::InsertChar('8'), Color::White), ("9", AppAction::InsertChar('9'), Color::White), ("×", AppAction::InsertChar('*'), Color::Yellow)],
        vec![("4", AppAction::InsertChar('4'), Color::White), ("5", AppAction::InsertChar('5'), Color::White), ("6", AppAction::InsertChar('6'), Color::White), ("-", AppAction::InsertChar('-'), Color::Yellow)],
        vec![("1", AppAction::InsertChar('1'), Color::White), ("2", AppAction::InsertChar('2'), Color::White), ("3", AppAction::InsertChar('3'), Color::White), ("+", AppAction::InsertChar('+'), Color::Yellow)],
        vec![("0", AppAction::InsertChar('0'), Color::White), (".", AppAction::InsertChar('.'), Color::White), ("=", AppAction::Evaluate, Color::Green)],
    ];

    let row_constraints = vec![Constraint::Ratio(1, 5); 5];
    let rows = Layout::vertical(row_constraints).split(area);

    for (r_idx, row_spec) in rows_spec.iter().enumerate() {
        let col_constraints: Vec<Constraint> = row_spec.iter().map(|_| Constraint::Ratio(1, row_spec.len() as u32)).collect();
        let cols = Layout::horizontal(col_constraints).split(rows[r_idx]);

        for (c_idx, (label, action, color)) in row_spec.iter().enumerate() {
            let cell = cols[c_idx];
            buttons_out.push(Button {
                label,
                action: action.clone(),
                area: cell,
                color: *color,
            });

            let btn_widget = Paragraph::new(Line::from(Span::styled(
                *label,
                Style::default().fg(*color).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(*color)),
            );

            frame.render_widget(btn_widget, cell);
        }
    }
}

pub fn render_advanced_keypad(frame: &mut Frame, area: Rect, buttons_out: &mut Vec<Button>) {
    let rows_spec = [
        vec![("sin", AppAction::InsertStr("sin(".to_string()), Color::Cyan), ("cos", AppAction::InsertStr("cos(".to_string()), Color::Cyan), ("tan", AppAction::InsertStr("tan(".to_string()), Color::Cyan), ("a/b", AppAction::InsertFraction, Color::LightMagenta)],
        vec![("asin", AppAction::InsertStr("asin(".to_string()), Color::Cyan), ("acos", AppAction::InsertStr("acos(".to_string()), Color::Cyan), ("atan", AppAction::InsertStr("atan(".to_string()), Color::Cyan), ("√", AppAction::InsertSqrt, Color::LightMagenta)],
        vec![("ln", AppAction::InsertStr("ln(".to_string()), Color::Cyan), ("log", AppAction::InsertStr("log(".to_string()), Color::Cyan), ("^", AppAction::InsertChar('^'), Color::Yellow), ("!", AppAction::InsertChar('!'), Color::Yellow)],
        vec![("det", AppAction::InsertStr("det(".to_string()), Color::Magenta), ("inv", AppAction::InsertStr("inv(".to_string()), Color::Magenta), ("eigen", AppAction::InsertStr("eigenval(".to_string()), Color::Magenta), ("norm", AppAction::InsertStr("norm(".to_string()), Color::Magenta)],
        vec![("pi", AppAction::InsertStr("pi".to_string()), Color::Green), ("e", AppAction::InsertStr("e".to_string()), Color::Green), ("ans", AppAction::InsertStr("ans".to_string()), Color::Green), ("%", AppAction::InsertChar('%'), Color::Yellow)],
    ];

    let row_constraints = vec![Constraint::Ratio(1, 5); 5];
    let rows = Layout::vertical(row_constraints).split(area);

    for (r_idx, row_spec) in rows_spec.iter().enumerate() {
        let col_constraints: Vec<Constraint> = row_spec.iter().map(|_| Constraint::Ratio(1, row_spec.len() as u32)).collect();
        let cols = Layout::horizontal(col_constraints).split(rows[r_idx]);

        for (c_idx, (label, action, color)) in row_spec.iter().enumerate() {
            let cell = cols[c_idx];
            buttons_out.push(Button {
                label,
                action: action.clone(),
                area: cell,
                color: *color,
            });

            let btn_widget = Paragraph::new(Line::from(Span::styled(
                *label,
                Style::default().fg(*color).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(*color)),
            );

            frame.render_widget(btn_widget, cell);
        }
    }
}
