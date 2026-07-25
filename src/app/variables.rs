use crate::eval::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VariableStore {
    vars: HashMap<String, Value>,
}

impl Default for VariableStore {
    fn default() -> Self {
        let mut vars = HashMap::new();
        // Initialize A-F with default Scalar(0.0)
        for c in 'A'..='F' {
            vars.insert(c.to_string(), Value::Scalar(0.0));
        }
        vars.insert("ans".to_string(), Value::Scalar(0.0));
        Self { vars }
    }
}

impl VariableStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    pub fn set_ans(&mut self, val: Value) {
        self.vars.insert("ans".to_string(), val);
    }

    pub fn get_ans(&self) -> Value {
        self.vars
            .get("ans")
            .cloned()
            .unwrap_or(Value::Scalar(0.0))
    }

    pub fn map(&self) -> &HashMap<String, Value> {
        &self.vars
    }
}
