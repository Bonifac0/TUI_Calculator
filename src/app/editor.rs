#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FractionSlot {
    Numerator,
    Denominator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerticalMove {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
enum ExprNode {
    Char(char),
    Fraction {
        numerator: Vec<ExprNode>,
        denominator: Vec<ExprNode>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CursorStep {
    fraction_index: usize,
    slot: FractionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EditorCursor {
    path: Vec<CursorStep>,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct EditorRender {
    pub cells: Vec<Vec<char>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

#[derive(Debug, Clone)]
pub struct EditorState {
    nodes: Vec<ExprNode>,
    cursor: EditorCursor,
}

const BACKSPACE_TOKENS: &[&str] = &[
    "eigenval(",
    "asin(",
    "acos(",
    "atan(",
    "sin(",
    "cos(",
    "tan(",
    "det(",
    "inv(",
    "norm(",
    "log(",
    "ln(",
    "ans",
    "pi",
    "√(",
];

impl Default for EditorState {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            cursor: EditorCursor {
                path: Vec::new(),
                index: 0,
            },
        }
    }
}

impl EditorState {
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.cursor.path.clear();
        self.cursor.index = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        let path = self.cursor.path.clone();
        let idx = self.cursor.index;
        let container = self.container_mut(&path);
        container.insert(idx, ExprNode::Char(c));
        self.cursor.index += 1;
    }

    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn insert_fraction(&mut self) {
        let path = self.cursor.path.clone();
        let idx = self.cursor.index;
        let container = self.container_mut(&path);

        let is_number_char = |node: &ExprNode| match node {
            ExprNode::Char(ch) => ch.is_ascii_digit() || *ch == '.',
            ExprNode::Fraction { .. } => false,
        };
        let is_variable_char = |node: &ExprNode| match node {
            ExprNode::Char(ch) => ('A'..='F').contains(ch),
            ExprNode::Fraction { .. } => false,
        };

        let mut start = idx;
        let mut end = idx;

        // Consume entire numeric token around the cursor (left and right).
        if (idx > 0 && is_number_char(&container[idx - 1]))
            || (idx < container.len() && is_number_char(&container[idx]))
        {
            while start > 0 && is_number_char(&container[start - 1]) {
                start -= 1;
            }
            while end < container.len() && is_number_char(&container[end]) {
                end += 1;
            }
        // Consume a single variable token A-F under/left of cursor.
        } else if idx > 0 && is_variable_char(&container[idx - 1]) {
            start = idx - 1;
            end = idx;
        } else if idx < container.len() && is_variable_char(&container[idx]) {
            start = idx;
            end = idx + 1;
        }

        let consumed = start < end;
        let numerator = if consumed {
            container.drain(start..end).collect()
        } else {
            Vec::new()
        };

        let fraction = ExprNode::Fraction {
            numerator,
            denominator: Vec::new(),
        };
        container.insert(start, fraction);

        self.cursor.path.push(CursorStep {
            fraction_index: start,
            slot: if consumed {
                FractionSlot::Denominator
            } else {
                FractionSlot::Numerator
            },
        });
        self.cursor.index = 0;
    }

    pub fn backspace(&mut self) {
        let path = self.cursor.path.clone();
        let idx = self.cursor.index;
        if idx == 0 {
            if self.collapse_empty_denominator_fraction() {
                return;
            }
            return;
        }

        let token_len = {
            let container = self.container(&path);
            Self::token_len_before_cursor(container, idx)
        };

        let container = self.container_mut(&path);
        if let Some(len) = token_len {
            let start = idx - len;
            container.drain(start..idx);
            self.cursor.index = start;
            return;
        }

        container.remove(idx - 1);
        self.cursor.index -= 1;
    }

    fn collapse_empty_denominator_fraction(&mut self) -> bool {
        let Some(step) = self.cursor.path.last().copied() else {
            return false;
        };
        if step.slot != FractionSlot::Denominator {
            return false;
        }

        let parent_path = self.cursor.path[..self.cursor.path.len() - 1].to_vec();
        let can_collapse = matches!(
            self.container(&parent_path).get(step.fraction_index),
            Some(ExprNode::Fraction { denominator, .. }) if denominator.is_empty()
        );
        if !can_collapse {
            return false;
        }

        let parent = self.container_mut(&parent_path);
        let numerator = match parent.remove(step.fraction_index) {
            ExprNode::Fraction { numerator, .. } => numerator,
            ExprNode::Char(_) => return false,
        };
        let restored_len = numerator.len();
        parent.splice(step.fraction_index..step.fraction_index, numerator);

        self.cursor.path.pop();
        self.cursor.index = step.fraction_index + restored_len;
        true
    }

    pub fn move_left(&mut self) {
        let idx = self.cursor.index;

        if idx > 0 {
            // Check if the node to the left is a fraction — enter its denominator at the end
            let den_entry = {
                let path = self.cursor.path.clone();
                let container = self.container(&path);
                if let ExprNode::Fraction { denominator, .. } = &container[idx - 1] {
                    Some((idx - 1, denominator.len()))
                } else {
                    None
                }
            };

            if let Some((fraction_index, den_len)) = den_entry {
                self.cursor.path.push(CursorStep {
                    fraction_index,
                    slot: FractionSlot::Denominator,
                });
                self.cursor.index = den_len;
            } else {
                self.cursor.index -= 1;
            }
            return;
        }

        // At start of current container → exit to parent, cursor before the fraction
        if let Some(step) = self.cursor.path.pop() {
            self.cursor.index = step.fraction_index;
        }
    }

    pub fn move_right(&mut self) {
        let idx = self.cursor.index;

        let (at_end, is_fraction) = {
            let path = self.cursor.path.clone();
            let container = self.container(&path);
            let len = container.len();
            if idx >= len {
                (true, false)
            } else {
                (false, matches!(container[idx], ExprNode::Fraction { .. }))
            }
        };

        if at_end {
            // At end of current container → exit to parent, cursor after the fraction
            if let Some(step) = self.cursor.path.pop() {
                self.cursor.index = step.fraction_index + 1;
            }
        } else if is_fraction {
            // Node to the right is a fraction — enter its numerator at the start
            self.cursor.path.push(CursorStep {
                fraction_index: idx,
                slot: FractionSlot::Numerator,
            });
            self.cursor.index = 0;
        } else {
            self.cursor.index += 1;
        }
    }

    pub fn move_home(&mut self) {
        self.cursor.index = 0;
    }

    pub fn move_end(&mut self) {
        let path = self.cursor.path.clone();
        self.cursor.index = self.container(&path).len();
    }

    pub fn move_up(&mut self) -> bool {
        if self.cursor.path.is_empty() {
            return self.enter_adjacent_fraction(FractionSlot::Numerator);
        }

        let last = self.cursor.path.last().copied().unwrap();
        if last.slot == FractionSlot::Numerator && self.cursor.path.len() > 1 {
            // Already in a numerator: go up to the parent fraction's numerator.
            let parent_step = self.cursor.path[self.cursor.path.len() - 2];
            let current_path = self.cursor.path[..self.cursor.path.len() - 1].to_vec();
            let mut target_path = self.cursor.path[..self.cursor.path.len() - 2].to_vec();
            target_path.push(CursorStep {
                fraction_index: parent_step.fraction_index,
                slot: FractionSlot::Numerator,
            });
            return self.relocate_between_slots(current_path, target_path, VerticalMove::Up);
        }

        self.switch_fraction_slot(FractionSlot::Numerator, VerticalMove::Up)
    }

    pub fn move_down(&mut self) -> bool {
        if self.cursor.path.is_empty() {
            return self.enter_adjacent_fraction(FractionSlot::Denominator);
        }

        let last = self.cursor.path.last().copied().unwrap();
        if last.slot == FractionSlot::Denominator && self.cursor.path.len() > 1 {
            // Already in a denominator: go down to the parent fraction's denominator.
            let parent_step = self.cursor.path[self.cursor.path.len() - 2];
            let current_path = self.cursor.path[..self.cursor.path.len() - 1].to_vec();
            let mut target_path = self.cursor.path[..self.cursor.path.len() - 2].to_vec();
            target_path.push(CursorStep {
                fraction_index: parent_step.fraction_index,
                slot: FractionSlot::Denominator,
            });
            return self.relocate_between_slots(current_path, target_path, VerticalMove::Down);
        }

        self.switch_fraction_slot(FractionSlot::Denominator, VerticalMove::Down)
    }

    pub fn to_plain_text(&self) -> String {
        fn write_nodes(nodes: &[ExprNode], out: &mut String) {
            for node in nodes {
                match node {
                    ExprNode::Char(ch) => out.push(*ch),
                    ExprNode::Fraction {
                        numerator,
                        denominator,
                    } => {
                        out.push_str("\\frac{");
                        write_nodes(numerator, out);
                        out.push_str("}{");
                        write_nodes(denominator, out);
                        out.push('}');
                    }
                }
            }
        }

        let mut out = String::new();
        write_nodes(&self.nodes, &mut out);
        out
    }

    pub fn render(&self) -> EditorRender {
        let rendered = self.render_container(&self.nodes, &[]);
        EditorRender {
            cells: rendered.cells,
            cursor_row: rendered.cursor_row.unwrap_or(0),
            cursor_col: rendered.cursor_col.unwrap_or(0),
        }
    }

    fn enter_adjacent_fraction(&mut self, slot: FractionSlot) -> bool {
        let path = self.cursor.path.clone();
        let idx = self.cursor.index;
        let container = self.container(&path);

        let target_idx = if idx < container.len() {
            matches!(container[idx], ExprNode::Fraction { .. }).then_some(idx)
        } else {
            None
        }
        .or_else(|| {
            if idx > 0 && matches!(container[idx - 1], ExprNode::Fraction { .. }) {
                Some(idx - 1)
            } else {
                None
            }
        });

        if let Some(fraction_index) = target_idx {
            self.cursor.path.push(CursorStep {
                fraction_index,
                slot,
            });
            self.cursor.index = 0;
            true
        } else {
            false
        }
    }

    fn switch_fraction_slot(&mut self, target_slot: FractionSlot, vertical_move: VerticalMove) -> bool {
        if self.cursor.path.is_empty() {
            return false;
        }

        let mut target_path = self.cursor.path.clone();
        if let Some(last) = target_path.last_mut() {
            last.slot = target_slot;
        }

        self.relocate_between_slots(self.cursor.path.clone(), target_path, vertical_move)
    }

    fn cursor_prefix_in_container(&self, container_path: &[CursorStep]) -> usize {
        if self.cursor.path == container_path {
            let boundaries = self.container_boundaries(container_path);
            return *boundaries
                .get(self.cursor.index)
                .unwrap_or(boundaries.last().unwrap_or(&0));
        }

        // If this is an ancestor container of the active cursor path, compute the visual prefix
        // by taking the cursor column inside the child subtree.
        if self.cursor.path.len() > container_path.len()
            && self.cursor.path[..container_path.len()] == *container_path
        {
            let nodes = self.container(container_path);
            let child_step = self.cursor.path[container_path.len()];
            if child_step.fraction_index < nodes.len() {
                let mut prefix = 0usize;
                for idx in 0..child_step.fraction_index {
                    prefix += self.render_node(&nodes[idx], container_path, idx).width;
                }
                let child_box =
                    self.render_node(&nodes[child_step.fraction_index], container_path, child_step.fraction_index);
                return prefix + child_box.cursor_col.unwrap_or(0);
            }
        }

        let boundaries = self.container_boundaries(container_path);
        *boundaries
            .get(self.cursor.index)
            .unwrap_or(boundaries.last().unwrap_or(&0))
    }

    fn relocate_between_slots(
        &mut self,
        current_path: Vec<CursorStep>,
        target_path: Vec<CursorStep>,
        vertical_move: VerticalMove,
    ) -> bool {
        let Some(current_step) = current_path.last().copied() else {
            return false;
        };
        let Some(target_step) = target_path.last().copied() else {
            return false;
        };

        let current_parent = &current_path[..current_path.len() - 1];
        let target_parent = &target_path[..target_path.len() - 1];
        if current_parent != target_parent || current_step.fraction_index != target_step.fraction_index {
            return false;
        }

        let parent_path = current_parent.to_vec();
        let num_path = {
            let mut p = parent_path.clone();
            p.push(CursorStep {
                fraction_index: current_step.fraction_index,
                slot: FractionSlot::Numerator,
            });
            p
        };
        let den_path = {
            let mut p = parent_path;
            p.push(CursorStep {
                fraction_index: current_step.fraction_index,
                slot: FractionSlot::Denominator,
            });
            p
        };

        let num_total = self.container_total_width(&num_path);
        let den_total = self.container_total_width(&den_path);
        let inner_w = num_total.max(den_total).max(1);

        let current_boundaries = self.container_boundaries(&current_path);
        let target_boundaries = self.container_boundaries(&target_path);

        let current_prefix = self.cursor_prefix_in_container(&current_path);
        let current_total = *current_boundaries.last().unwrap_or(&0);
        let target_total = *target_boundaries.last().unwrap_or(&0);

        let current_left = 1 + (inner_w.saturating_sub(current_total)) / 2;
        let target_left = 1 + (inner_w.saturating_sub(target_total)) / 2;
        let absolute_x = current_left + current_prefix;

        let target_prefix = if absolute_x <= target_left {
            0
        } else {
            (absolute_x - target_left).min(target_total)
        };

        let (resolved_path, resolved_index) =
            self.resolve_vertical_target(&target_path, target_prefix, vertical_move);
        self.cursor.path = resolved_path;
        self.cursor.index = resolved_index;
        true
    }

    fn resolve_vertical_target(
        &self,
        start_path: &[CursorStep],
        start_prefix: usize,
        vertical_move: VerticalMove,
    ) -> (Vec<CursorStep>, usize) {
        let mut current_path = start_path.to_vec();
        let mut current_prefix = start_prefix;

        loop {
            let boundaries = self.container_boundaries(&current_path);
            let nodes = self.container(&current_path);

            if nodes.is_empty() {
                return (current_path, 0);
            }

            let mut hit_fraction: Option<(usize, usize)> = None;
            for idx in 0..nodes.len() {
                let start = boundaries[idx];
                let end = boundaries[idx + 1];
                if current_prefix > start && current_prefix < end {
                    if matches!(nodes[idx], ExprNode::Fraction { .. }) {
                        hit_fraction = Some((idx, start));
                    }
                    break;
                }
            }

            if let Some((fraction_index, fraction_start)) = hit_fraction {
                let slot = match vertical_move {
                    VerticalMove::Down => FractionSlot::Numerator,
                    VerticalMove::Up => FractionSlot::Denominator,
                };

                let num_path = {
                    let mut p = current_path.clone();
                    p.push(CursorStep {
                        fraction_index,
                        slot: FractionSlot::Numerator,
                    });
                    p
                };
                let den_path = {
                    let mut p = current_path.clone();
                    p.push(CursorStep {
                        fraction_index,
                        slot: FractionSlot::Denominator,
                    });
                    p
                };

                let num_total = self.container_total_width(&num_path);
                let den_total = self.container_total_width(&den_path);
                let inner_w = num_total.max(den_total).max(1);

                let mut next_path = current_path.clone();
                next_path.push(CursorStep { fraction_index, slot });
                let slot_total = self.container_total_width(&next_path);
                let slot_left = 1 + (inner_w.saturating_sub(slot_total)) / 2;

                let local_x = current_prefix.saturating_sub(fraction_start);
                current_prefix = if local_x <= slot_left {
                    0
                } else {
                    (local_x - slot_left).min(slot_total)
                };
                current_path = next_path;
                continue;
            }

            let target_index = nearest_boundary_index(&boundaries, current_prefix);
            return (current_path, target_index);
        }
    }

    fn container_boundaries(&self, path: &[CursorStep]) -> Vec<usize> {
        let nodes = self.container(path);
        let mut boundaries = Vec::with_capacity(nodes.len() + 1);
        boundaries.push(0);
        let mut sum = 0usize;
        for (idx, node) in nodes.iter().enumerate() {
            sum += self.render_node(node, path, idx).width;
            boundaries.push(sum);
        }
        boundaries
    }

    fn container_total_width(&self, path: &[CursorStep]) -> usize {
        *self.container_boundaries(path).last().unwrap_or(&0)
    }

    fn token_len_before_cursor(container: &[ExprNode], cursor_index: usize) -> Option<usize> {
        let mut chars = Vec::new();
        let mut i = cursor_index;
        while i > 0 {
            match container[i - 1] {
                ExprNode::Char(ch) => {
                    chars.push(ch);
                    i -= 1;
                }
                ExprNode::Fraction { .. } => break,
            }
        }

        chars.reverse();
        let before: String = chars.into_iter().collect();

        BACKSPACE_TOKENS
            .iter()
            .filter_map(|token| before.ends_with(token).then_some(token.chars().count()))
            .max()
    }

    fn container<'a>(&'a self, path: &[CursorStep]) -> &'a [ExprNode] {
        fn descend<'a>(nodes: &'a [ExprNode], path: &[CursorStep]) -> &'a [ExprNode] {
            if let Some((head, tail)) = path.split_first() {
                match &nodes[head.fraction_index] {
                    ExprNode::Fraction {
                        numerator,
                        denominator,
                    } => match head.slot {
                        FractionSlot::Numerator => descend(numerator, tail),
                        FractionSlot::Denominator => descend(denominator, tail),
                    },
                    ExprNode::Char(_) => panic!("Invalid cursor path: char is not a container"),
                }
            } else {
                nodes
            }
        }

        descend(&self.nodes, path)
    }

    fn container_mut<'a>(&'a mut self, path: &[CursorStep]) -> &'a mut Vec<ExprNode> {
        fn descend<'a>(nodes: &'a mut Vec<ExprNode>, path: &[CursorStep]) -> &'a mut Vec<ExprNode> {
            if let Some((head, tail)) = path.split_first() {
                match &mut nodes[head.fraction_index] {
                    ExprNode::Fraction {
                        numerator,
                        denominator,
                    } => match head.slot {
                        FractionSlot::Numerator => descend(numerator, tail),
                        FractionSlot::Denominator => descend(denominator, tail),
                    },
                    ExprNode::Char(_) => panic!("Invalid cursor path: char is not a container"),
                }
            } else {
                nodes
            }
        }

        descend(&mut self.nodes, path)
    }

    fn render_container(&self, nodes: &[ExprNode], path: &[CursorStep]) -> RenderBox {
        let mut node_boxes = Vec::new();
        for (idx, node) in nodes.iter().enumerate() {
            node_boxes.push(self.render_node(node, path, idx));
        }

        let content_width: usize = node_boxes.iter().map(|b| b.width).sum();
        let baseline = node_boxes.iter().map(|b| b.baseline).max().unwrap_or(0);
        let mut height = node_boxes
            .iter()
            .map(|b| baseline + b.height.saturating_sub(b.baseline))
            .max()
            .unwrap_or(1);
        if height == 0 {
            height = 1;
        }
        let width = content_width.max(1);

        let mut cells = vec![vec![' '; width]; height];
        let mut cursor_row = None;
        let mut cursor_col = None;

        let mut x_offset = 0;
        for b in node_boxes {
            let y_offset = baseline.saturating_sub(b.baseline);
            for (row_idx, row) in b.cells.iter().enumerate() {
                for (col_idx, ch) in row.iter().enumerate() {
                    cells[y_offset + row_idx][x_offset + col_idx] = *ch;
                }
            }
            if let (Some(r), Some(c)) = (b.cursor_row, b.cursor_col) {
                cursor_row = Some(y_offset + r);
                cursor_col = Some(x_offset + c);
            }
            x_offset += b.width;
        }

        if self.cursor.path.as_slice() == path {
            let before_width: usize = node_boxes_width_prefix(nodes, self, path, self.cursor.index);
            cursor_row = Some(baseline);
            cursor_col = Some(before_width);
        }

        RenderBox {
            cells,
            width,
            height,
            baseline,
            cursor_row,
            cursor_col,
        }
    }

    fn render_node(&self, node: &ExprNode, path: &[CursorStep], idx: usize) -> RenderBox {
        match node {
            ExprNode::Char(ch) => RenderBox {
                cells: vec![vec![*ch]],
                width: 1,
                height: 1,
                baseline: 0,
                cursor_row: None,
                cursor_col: None,
            },
            ExprNode::Fraction {
                numerator,
                denominator,
            } => {
                let mut num_path = path.to_vec();
                num_path.push(CursorStep {
                    fraction_index: idx,
                    slot: FractionSlot::Numerator,
                });
                let num = self.render_container(numerator, &num_path);

                let mut den_path = path.to_vec();
                den_path.push(CursorStep {
                    fraction_index: idx,
                    slot: FractionSlot::Denominator,
                });
                let den = self.render_container(denominator, &den_path);

                let inner_w = num.width.max(den.width).max(1);
                let width = inner_w + 2;
                let height = num.height + 1 + den.height;
                let baseline = num.height;
                let mut cells = vec![vec![' '; width]; height];

                let num_x = 1 + (inner_w - num.width) / 2;
                for (r, row) in num.cells.iter().enumerate() {
                    for (c, ch) in row.iter().enumerate() {
                        cells[r][num_x + c] = *ch;
                    }
                }

                for c in 0..width {
                    cells[num.height][c] = '-';
                }

                let den_x = 1 + (inner_w - den.width) / 2;
                for (r, row) in den.cells.iter().enumerate() {
                    for (c, ch) in row.iter().enumerate() {
                        cells[num.height + 1 + r][den_x + c] = *ch;
                    }
                }

                let (cursor_row, cursor_col) = if let (Some(r), Some(c)) = (num.cursor_row, num.cursor_col) {
                    (Some(r), Some(num_x + c))
                } else if let (Some(r), Some(c)) = (den.cursor_row, den.cursor_col) {
                    (Some(num.height + 1 + r), Some(den_x + c))
                } else {
                    (None, None)
                };

                RenderBox {
                    cells,
                    width,
                    height,
                    baseline,
                    cursor_row,
                    cursor_col,
                }
            }
        }
    }
}

fn node_boxes_width_prefix(
    nodes: &[ExprNode],
    editor: &EditorState,
    path: &[CursorStep],
    cursor_index: usize,
) -> usize {
    let mut width = 0;
    for (idx, node) in nodes.iter().enumerate().take(cursor_index) {
        width += editor.render_node(node, path, idx).width;
    }
    width
}

#[derive(Debug, Clone)]
struct RenderBox {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
    baseline: usize,
    cursor_row: Option<usize>,
    cursor_col: Option<usize>,
}

fn nearest_boundary_index(boundaries: &[usize], target: usize) -> usize {
    let mut best_idx = 0usize;
    let mut best_dist = usize::MAX;
    for (idx, &value) in boundaries.iter().enumerate() {
        let dist = value.abs_diff(target);
        if dist < best_dist {
            best_dist = dist;
            best_idx = idx;
        }
    }
    best_idx
}
