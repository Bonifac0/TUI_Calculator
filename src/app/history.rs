#[derive(Debug, Clone, Default)]
pub struct History {
    items: Vec<(String, String)>,
}

impl History {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, expr: String, result: String) {
        self.items.push((expr, result));
    }

    pub fn items(&self) -> &[(String, String)] {
        &self.items
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
