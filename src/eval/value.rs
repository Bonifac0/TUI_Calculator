use nalgebra::{DMatrix, DVector};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(f64),
    Matrix(DMatrix<f64>),
    Vector(DVector<f64>),
}

impl Value {
    pub fn to_formatted_string(&self, precision: usize) -> String {
        match self {
            Value::Scalar(val) => {
                if val.is_nan() {
                    "NaN".to_string()
                } else if val.is_infinite() {
                    if *val > 0.0 { "∞".to_string() } else { "-∞".to_string() }
                } else if val.fract() == 0.0 && val.abs() < 1e15 {
                    format!("{:.0}", val)
                } else {
                    let s = format!("{:.1$}", val, precision);
                    s.trim_end_matches('0').trim_end_matches('.').to_string()
                }
            }
            Value::Matrix(mat) => {
                let rows = mat.nrows();
                let cols = mat.ncols();
                let mut lines = Vec::new();
                for r in 0..rows {
                    let row_vals: Vec<String> = (0..cols)
                        .map(|c| {
                            let v = mat[(r, c)];
                            if v.fract() == 0.0 {
                                format!("{:.0}", v)
                            } else {
                                format!("{:.1$}", v, precision)
                                    .trim_end_matches('0')
                                    .trim_end_matches('.')
                                    .to_string()
                            }
                        })
                        .collect();
                    lines.push(format!("[ {} ]", row_vals.join(", ")));
                }
                format!("[ {} ]", lines.join(" ; "))
            }
            Value::Vector(vec) => {
                let elements: Vec<String> = vec
                    .iter()
                    .map(|&v| {
                        if v.fract() == 0.0 {
                            format!("{:.0}", v)
                        } else {
                            format!("{:.1$}", v, precision)
                                .trim_end_matches('0')
                                .trim_end_matches('.')
                                .to_string()
                        }
                    })
                    .collect();
                format!("[ {} ]", elements.join(", "))
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_formatted_string(6))
    }
}
