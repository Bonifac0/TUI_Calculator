pub mod app;
pub mod config;
pub mod eval;
pub mod input;
pub mod parser;
pub mod ui;

use app::App;
use clap::Parser as ClapParser;
use config::{Cli, Settings};
use eval::Evaluator;
use input::{poll_action, AppAction};
use parser::parse_expression;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    // Check for non-interactive execution mode
    if let Some(expr_str) = cli.expression {
        let settings = Settings::load();
        let vars = app::VariableStore::new();

        match parse_expression(&expr_str) {
            Ok(ast) => {
                let evaluator = Evaluator::new(vars.map(), settings.angle_unit);
                match evaluator.eval(&ast) {
                    Ok(val) => {
                        println!("{}", val.to_formatted_string(settings.precision));
                        std::process::exit(0);
                    }
                    Err(err) => {
                        eprintln!("Evaluation Error: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                eprintln!("Parse Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    // Launch TUI mode
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut app = App::new();

    // Enforce CLI layout overrides if specified
    if cli.small {
        app.debug_log = "Layout forced: Small".to_string();
    } else if cli.medium {
        app.debug_log = "Layout forced: Medium".to_string();
    } else if cli.big {
        app.debug_log = "Layout forced: Big".to_string();
    }

    let mut buttons = Vec::new();

    while !app.exit {
        terminal.draw(|frame| {
            buttons = ui::draw(frame, &mut app);
        })?;

        if let Some(action) = poll_action()? {
            execute_action(&mut app, action, &buttons);
        }
    }

    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture);
    ratatui::restore();
    Ok(())
}

fn execute_action(app: &mut App, action: AppAction, buttons: &[ui::keypad::Button]) {
    match action {
        AppAction::InsertChar(c) => app.insert_char(c),
        AppAction::InsertStr(s) => app.insert_str(&s),
        AppAction::InsertFraction => app.insert_fraction(),
        AppAction::InsertSqrt => app.insert_sqrt(),
        AppAction::Backspace => app.backspace(),
        AppAction::MoveCursorLeft => app.move_cursor_left(),
        AppAction::MoveCursorRight => app.move_cursor_right(),
        AppAction::MoveCursorHome => app.move_cursor_home(),
        AppAction::MoveCursorEnd => app.move_cursor_end(),
        AppAction::ScrollUp => app.scroll_up(),
        AppAction::ScrollDown => app.scroll_down(),
        AppAction::Evaluate => app.evaluate(),
        AppAction::ClearInput => app.clear_input(),
        AppAction::AllClear => app.all_clear(),
        AppAction::StoreVariable(v) => app.store_variable(v),
        AppAction::InsertVariable(v) => app.insert_char(v),
        AppAction::ToggleAngleUnit => app.toggle_angle_unit(),
        AppAction::OpenHelp => app.show_help = !app.show_help,
        AppAction::CloseHelp => app.show_help = false,
        AppAction::Quit => app.exit = true,
        AppAction::ClickAt(col, row) => {
            if app.show_help {
                app.show_help = false;
                return;
            }
            for btn in buttons {
                if col >= btn.area.x
                    && col < btn.area.x + btn.area.width
                    && row >= btn.area.y
                    && row < btn.area.y + btn.area.height
                {
                    execute_action(app, btn.action.clone(), buttons);
                    break;
                }
            }
        }
    }
}
