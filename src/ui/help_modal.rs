use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect) {
    // Center modal dialog
    let modal_area = centered_rect(70, 70, area);

    let text = vec![
        Line::from(Span::styled("TUI Calculator Help & Quick Reference", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled("Keybindings & Controls:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from("  • Type Numbers & Operators (+, -, *, /, ^, !) directly from keyboard"),
        Line::from("  • Enter or '='       : Evaluate expression"),
        Line::from("  • Backspace         : Delete character before cursor"),
        Line::from("  • Left / Right      : Move editing cursor"),
        Line::from("  • Up / Down         : Scroll 2D math canvas"),
        Line::from("  • 'c' / 'C'         : Clear active expression"),
        Line::from("  • Ctrl+C            : All Clear (Reset memory & variables)"),
        Line::from("  • 'u' / 'U'         : Toggle Angle Unit (DEG <-> RAD)"),
        Line::from("  • Shift + <A-F>     : Store 'ans' into Variable A-F"),
        Line::from("  • <A-F>             : Insert Variable A-F into expression"),
        Line::from("  • '?' / F1          : Toggle Help Modal"),
        Line::from("  • 'q' / Esc         : Quit application"),
        Line::from(""),
        Line::from(Span::styled("LaTeX Syntax Support:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from("  • Fractions         : \\frac{numerator}{denominator}"),
        Line::from("  • Square Root       : \\sqrt{expression}"),
        Line::from("  • Inline Matrices   : [[1, 2], [3, 4]] or [1, 2; 3, 4]"),
        Line::from("  • Matrix Functions  : det(A), inv(A), eigenval(A), norm(V)"),
        Line::from(""),
        Line::from(Span::styled("Press '?' or Esc to Close Help", Style::default().fg(Color::Green))),
    ];

    let block = Block::bordered()
        .title(" Help & Usage ")
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Left)
        .block(block);

    frame.render_widget(Clear, modal_area); // Clear background behind modal
    frame.render_widget(paragraph, modal_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
