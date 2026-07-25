use color_eyre::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    
    let app_result = App::default().run(&mut terminal);

    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture);
    ratatui::restore();
    app_result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Digit(char),
    Op(char),
    Decimal,
    Evaluate,
    Clear,
    AllClear,
    Backspace,
    Quit,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub label: String,
    pub action: Action,
    pub area: Rect,
    pub style_type: ButtonStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Number,
    Operator,
    Action,
    Equals,
}

#[derive(Debug, Clone, Default)]
pub struct App {
    pub input: String,
    pub result: String,
    pub history: Vec<(String, String)>,
    pub error_message: Option<String>,
    pub exit: bool,
    pub buttons: Vec<Button>,
    pub selected_button: usize,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Main layout: Header, Body, Footer
        let chunks = Layout::vertical([
            Constraint::Length(3), // Title & Quick Help
            Constraint::Min(12),   // Calculator display + History + Buttons
            Constraint::Length(1), // Status / Error bar
        ])
        .split(area);

        // Header
        let header = Paragraph::new(Line::from(vec![
            Span::styled(" TUI Calculator ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" │ "),
            Span::styled("Type numbers & ops or Click buttons", Style::default().fg(Color::DarkGray)),
            Span::raw(" │ "),
            Span::styled("Press 'q' or Esc to Quit", Style::default().fg(Color::Yellow)),
        ]))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(header, chunks[0]);

        // Body split into Left (Calculator) and Right (History)
        let body_chunks = Layout::horizontal([
            Constraint::Percentage(60), // Calculator Display & Grid
            Constraint::Percentage(40), // History Log
        ])
        .split(chunks[1]);

        // Left Body: Display (Top) + Keypad (Bottom)
        let calc_chunks = Layout::vertical([
            Constraint::Length(5), // Display Box (Expression + Result)
            Constraint::Min(8),    // Button Grid
        ])
        .split(body_chunks[0]);

        // Display Box
        let status_color = if self.error_message.is_some() {
            Color::Red
        } else {
            Color::Green
        };

        let display_lines = vec![
            Line::from(vec![
                Span::styled("Expr: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    if self.input.is_empty() { "0" } else { &self.input },
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Result: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    if self.result.is_empty() { "=" } else { &self.result },
                    Style::default().fg(status_color).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let display_box = Paragraph::new(display_lines)
            .alignment(Alignment::Right)
            .block(
                Block::bordered()
                    .title(" Display ")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Blue)),
            );
        frame.render_widget(display_box, calc_chunks[0]);

        // Keypad Grid Render
        self.render_keypad(frame, calc_chunks[1]);

        // Right Body: History
        let history_items: Vec<ListItem> = self
            .history
            .iter()
            .rev()
            .map(|(expr, res)| {
                ListItem::new(vec![
                    Line::from(Span::styled(expr, Style::default().fg(Color::Gray))),
                    Line::from(vec![
                        Span::raw("= "),
                        Span::styled(res, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    ]),
                ])
            })
            .collect();

        let history_list = List::new(history_items).block(
            Block::bordered()
                .title(" History ")
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Magenta)),
        );
        frame.render_widget(history_list, body_chunks[1]);

        // Footer / Error Bar
        let footer_text = if let Some(ref err) = self.error_message {
            Line::from(Span::styled(format!(" Error: {}", err), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(Span::styled(" Ready", Style::default().fg(Color::DarkGray)))
        };
        let footer = Paragraph::new(footer_text);
        frame.render_widget(footer, chunks[2]);
    }

    fn render_keypad(&mut self, frame: &mut Frame, area: Rect) {
        let row_defs = [
            vec![("C", Action::Clear, ButtonStyle::Action), ("AC", Action::AllClear, ButtonStyle::Action), ("⌫", Action::Backspace, ButtonStyle::Action), ("÷", Action::Op('/'), ButtonStyle::Operator)],
            vec![("7", Action::Digit('7'), ButtonStyle::Number), ("8", Action::Digit('8'), ButtonStyle::Number), ("9", Action::Digit('9'), ButtonStyle::Number), ("×", Action::Op('*'), ButtonStyle::Operator)],
            vec![("4", Action::Digit('4'), ButtonStyle::Number), ("5", Action::Digit('5'), ButtonStyle::Number), ("6", Action::Digit('6'), ButtonStyle::Number), ("-", Action::Op('-'), ButtonStyle::Operator)],
            vec![("1", Action::Digit('1'), ButtonStyle::Number), ("2", Action::Digit('2'), ButtonStyle::Number), ("3", Action::Digit('3'), ButtonStyle::Number), ("+", Action::Op('+'), ButtonStyle::Operator)],
            vec![("0", Action::Digit('0'), ButtonStyle::Number), (".", Action::Decimal, ButtonStyle::Number), ("=", Action::Evaluate, ButtonStyle::Equals)],
        ];

        // 5 rows total
        let rows = Layout::vertical([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
        ])
        .split(area);

        self.buttons.clear();

        for (row_idx, row_spec) in row_defs.iter().enumerate() {
            let col_constraints: Vec<Constraint> = row_spec.iter().map(|_| Constraint::Ratio(1, row_spec.len() as u32)).collect();
            let cols = Layout::horizontal(col_constraints).split(rows[row_idx]);

            for (col_idx, (label, action, style_type)) in row_spec.iter().enumerate() {
                let cell_area = cols[col_idx];
                self.buttons.push(Button {
                    label: label.to_string(),
                    action: *action,
                    area: cell_area,
                    style_type: *style_type,
                });

                let (bg_color, fg_color) = match style_type {
                    ButtonStyle::Number => (Color::Reset, Color::White),
                    ButtonStyle::Operator => (Color::Reset, Color::Yellow),
                    ButtonStyle::Action => (Color::Reset, Color::Red),
                    ButtonStyle::Equals => (Color::Reset, Color::Green),
                };

                let btn_widget = Paragraph::new(Line::from(Span::styled(
                    *label,
                    Style::default().fg(fg_color).add_modifier(Modifier::BOLD),
                )))
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(bg_color)),
                );

                frame.render_widget(btn_widget, cell_area);
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key(key),
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            _ => {}
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        self.error_message = None;

        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.execute_action(Action::Quit),
            KeyCode::Char(c) if c.is_ascii_digit() => self.execute_action(Action::Digit(c)),
            KeyCode::Char('+') => self.execute_action(Action::Op('+')),
            KeyCode::Char('-') => self.execute_action(Action::Op('-')),
            KeyCode::Char('*') | KeyCode::Char('x') => self.execute_action(Action::Op('*')),
            KeyCode::Char('/') => self.execute_action(Action::Op('/')),
            KeyCode::Char('.') => self.execute_action(Action::Decimal),
            KeyCode::Enter | KeyCode::Char('=') => self.execute_action(Action::Evaluate),
            KeyCode::Backspace => self.execute_action(Action::Backspace),
            KeyCode::Char('c') | KeyCode::Char('C') => self.execute_action(Action::Clear),
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            let column = mouse.column;
            let row = mouse.row;

            for btn in &self.buttons {
                if column >= btn.area.x
                    && column < btn.area.x + btn.area.width
                    && row >= btn.area.y
                    && row < btn.area.y + btn.area.height
                {
                    self.execute_action(btn.action);
                    break;
                }
            }
        }
    }

    pub fn execute_action(&mut self, action: Action) {
        self.error_message = None;

        match action {
            Action::Digit(d) => {
                self.input.push(d);
            }
            Action::Op(op) => {
                if !self.input.is_empty() {
                    let last_char = self.input.chars().last().unwrap();
                    if "+-*/".contains(last_char) {
                        self.input.pop();
                    }
                    self.input.push(op);
                } else if op == '-' {
                    // allow negative leading number
                    self.input.push('-');
                }
            }
            Action::Decimal => {
                if !self.input.ends_with('.') {
                    self.input.push('.');
                }
            }
            Action::Evaluate => {
                self.evaluate();
            }
            Action::Clear => {
                self.input.clear();
            }
            Action::AllClear => {
                self.input.clear();
                self.result.clear();
                self.history.clear();
            }
            Action::Backspace => {
                self.input.pop();
            }
            Action::Quit => {
                self.exit = true;
            }
        }
    }

    fn evaluate(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        match eval_expression(&self.input) {
            Ok(val) => {
                let formatted = if val.fract() == 0.0 {
                    format!("{:.0}", val)
                } else {
                    format!("{:.4}", val).trim_end_matches('0').trim_end_matches('.').to_string()
                };

                self.result = formatted.clone();
                self.history.push((self.input.clone(), formatted));
            }
            Err(err) => {
                self.error_message = Some(err);
            }
        }
    }
}

/// Simple expression evaluator for basic math operations (+, -, *, /).
fn eval_expression(expr: &str) -> Result<f64, String> {
    let clean_expr = expr.replace('×', "*").replace('÷', "/");
    
    // Basic tokenizer / solver for simple sequential arithmetic
    // Handles left-to-right operations with standard precedence (* / over + -)
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();

    for ch in clean_expr.chars() {
        if "+-*/".contains(ch) {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            } else if ch == '-' && tokens.is_empty() {
                // Leading negative
                current.push('-');
                continue;
            }
            tokens.push(ch.to_string());
        } else if ch.is_ascii_digit() || ch == '.' {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    if tokens.is_empty() {
        return Err("Empty expression".into());
    }

    // Step 1: Handle * and /
    let mut idx = 0;
    while idx < tokens.len() {
        if tokens[idx] == "*" || tokens[idx] == "/" {
            if idx == 0 || idx + 1 >= tokens.len() {
                return Err("Syntax error".into());
            }
            let left: f64 = tokens[idx - 1].parse().map_err(|_| "Invalid number")?;
            let right: f64 = tokens[idx + 1].parse().map_err(|_| "Invalid number")?;
            
            let res = if tokens[idx] == "*" {
                left * right
            } else {
                if right == 0.0 {
                    return Err("Divide by zero".into());
                }
                left / right
            };

            tokens[idx - 1] = res.to_string();
            tokens.remove(idx); // remove op
            tokens.remove(idx); // remove right
            idx = idx.saturating_sub(1);
        } else {
            idx += 1;
        }
    }

    // Step 2: Handle + and -
    let mut total: f64 = tokens[0].parse().map_err(|_| "Invalid number")?;
    let mut idx = 1;
    while idx + 1 < tokens.len() {
        let op = &tokens[idx];
        let next_val: f64 = tokens[idx + 1].parse().map_err(|_| "Invalid number")?;

        if op == "+" {
            total += next_val;
        } else if op == "-" {
            total -= next_val;
        } else {
            return Err("Syntax error".into());
        }
        idx += 2;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple() {
        assert_eq!(eval_expression("2+3").unwrap(), 5.0);
        assert_eq!(eval_expression("10-4").unwrap(), 6.0);
        assert_eq!(eval_expression("3*4").unwrap(), 12.0);
        assert_eq!(eval_expression("12/4").unwrap(), 3.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(eval_expression("2+3*4").unwrap(), 14.0);
        assert_eq!(eval_expression("10-6/2").unwrap(), 7.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(eval_expression("5/0").is_err());
    }
}
