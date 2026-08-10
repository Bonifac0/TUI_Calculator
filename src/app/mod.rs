pub mod editor;
pub mod history;
pub mod variables;

pub use history::History;
pub use variables::VariableStore;

use crate::config::{AngleUnit, Settings};
use crate::eval::{Evaluator, Value};
use crate::parser::parse_expression;
use editor::{EditorRender, EditorState};

#[derive(Debug, Clone)]
pub struct App {
    editor: EditorState,
    pub input: String,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub result: String,
    pub last_value: Option<Value>,
    pub history: History,
    pub variables: VariableStore,
    pub angle_unit: AngleUnit,
    pub precision: usize,
    pub error_message: Option<String>,
    pub debug_log: String,
    pub show_help: bool,
    pub exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let settings = Settings::load();
        Self {
            editor: EditorState::default(),
            input: String::new(),
            scroll_x: 0,
            scroll_y: 0,
            result: String::new(),
            last_value: None,
            history: History::new(),
            variables: VariableStore::new(),
            angle_unit: settings.angle_unit,
            precision: settings.precision,
            error_message: None,
            debug_log: "System initialized. Ready.".to_string(),
            show_help: false,
            exit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn warn(&mut self, msg: String) {
        self.debug_log = format!("Warning: {}", msg);
    }

    fn clear_transient_state(&mut self) {
        self.error_message = None;
        if self.debug_log.starts_with("Warning:") {
            self.debug_log = "Ready".to_string();
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.clear_transient_state();
        self.editor.insert_char(c);
        self.sync_input_snapshot();
    }

    pub fn insert_str(&mut self, s: &str) {
        self.clear_transient_state();
        self.editor.insert_str(s);
        self.sync_input_snapshot();
    }

    pub fn insert_fraction(&mut self) {
        self.clear_transient_state();
        self.editor.insert_fraction();
        self.sync_input_snapshot();
    }

    pub fn insert_sqrt(&mut self) {
        self.clear_transient_state();
        self.editor.insert_str("√(");
        self.sync_input_snapshot();
    }

    pub fn backspace(&mut self) {
        self.clear_transient_state();
        self.editor.backspace();
        self.sync_input_snapshot();
    }

    pub fn move_cursor_left(&mut self) {
        self.editor.move_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.editor.move_right();
    }

    pub fn move_cursor_home(&mut self) {
        self.editor.move_home();
    }

    pub fn move_cursor_end(&mut self) {
        self.editor.move_end();
    }

    pub fn scroll_up(&mut self) {
        if !self.editor.move_up() && self.scroll_y > 0 {
            self.scroll_y -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if !self.editor.move_down() {
            self.scroll_y += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.editor.clear();
        self.sync_input_snapshot();
        self.scroll_x = 0;
        self.scroll_y = 0;
        self.error_message = None;
    }

    pub fn all_clear(&mut self) {
        self.clear_input();
        self.result.clear();
        self.last_value = None;
        self.history.clear();
        self.variables = VariableStore::new();
        self.debug_log = "All memory cleared.".to_string();
    }

    pub fn toggle_angle_unit(&mut self) {
        self.angle_unit = match self.angle_unit {
            AngleUnit::Deg => AngleUnit::Rad,
            AngleUnit::Rad => AngleUnit::Deg,
        };
        self.debug_log = format!("Angle unit set to {:?}", self.angle_unit);
    }

    pub fn store_variable(&mut self, var_name: char) {
        let name = var_name.to_string().to_uppercase();
        if let Some(ref val) = self.last_value {
            self.variables.set(name.clone(), val.clone());
            self.debug_log = format!("Stored {} in variable {}", val, name);
        } else {
            let ans = self.variables.get_ans();
            self.variables.set(name.clone(), ans.clone());
            self.debug_log = format!("Stored ans ({}) in variable {}", ans, name);
        }
    }

    pub fn evaluate(&mut self) {
        self.error_message = None;
        self.sync_input_snapshot();
        let expr = self.input.clone();
        let expr_str = expr.trim();
        if expr_str.is_empty() {
            return;
        }

        match parse_expression(expr_str) {
            Ok(ast) => {
                let evaluator = Evaluator::new(self.variables.map(), self.angle_unit);
                match evaluator.eval(&ast) {
                    Ok(val) => {
                        let formatted = val.to_formatted_string(self.precision);
                        self.result = formatted.clone();
                        self.last_value = Some(val.clone());
                        self.variables.set_ans(val);
                        self.history.push(expr_str.to_string(), formatted);
                        self.debug_log = format!("Evaluated: {}", expr_str);
                    }
                    Err(err) => {
                        self.error_message = Some(err.clone());
                        self.debug_log = format!("Eval Error: {}", err);
                    }
                }
            }
            Err(err) => {
                self.error_message = Some(err.clone());
                self.debug_log = format!("Parse Error: {}", err);
            }
        }
    }

    pub fn render_expression(&self) -> EditorRender {
        self.editor.render()
    }

    fn sync_input_snapshot(&mut self) {
        self.input = self.editor.to_plain_text();
    }
}
