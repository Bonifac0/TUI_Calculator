pub mod linear_algebra;
pub mod value;

pub use value::Value;

use crate::config::AngleUnit;
use crate::parser::ast::Node;
use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;

pub struct Evaluator<'a> {
    variables: &'a HashMap<String, Value>,
    angle_unit: AngleUnit,
}

impl<'a> Evaluator<'a> {
    pub fn new(variables: &'a HashMap<String, Value>, angle_unit: AngleUnit) -> Self {
        Self {
            variables,
            angle_unit,
        }
    }

    pub fn eval(&self, node: &Node) -> Result<Value, String> {
        match node {
            Node::Number(val) => Ok(Value::Scalar(*val)),
            Node::Variable(name) => {
                let name_lower = name.to_lowercase();
                if name_lower == "pi" {
                    return Ok(Value::Scalar(std::f64::consts::PI));
                } else if name_lower == "e" {
                    return Ok(Value::Scalar(std::f64::consts::E));
                }

                if let Some(val) = self.variables.get(name) {
                    Ok(val.clone())
                } else if let Some(val) = self.variables.get(&name.to_uppercase()) {
                    Ok(val.clone())
                } else {
                    Err(format!("Undefined variable: '{}'", name))
                }
            }
            Node::BinaryOp { op, left, right } => {
                let l_val = self.eval(left)?;
                let r_val = self.eval(right)?;
                self.eval_binary_op(*op, &l_val, &r_val)
            }
            Node::UnaryOp { op, expr } => {
                let val = self.eval(expr)?;
                match op {
                    '-' => match val {
                        Value::Scalar(s) => Ok(Value::Scalar(-s)),
                        Value::Matrix(m) => Ok(Value::Matrix(-m)),
                        Value::Vector(v) => Ok(Value::Vector(-v)),
                    },
                    _ => Err(format!("Unsupported unary operator: '{}'", op)),
                }
            }
            Node::Fraction { numerator, denominator } => {
                let num_val = self.eval(numerator)?;
                let den_val = self.eval(denominator)?;
                self.eval_binary_op('/', &num_val, &den_val)
            }
            Node::FunctionCall { name, args } => self.eval_function(name, args),
            Node::Factorial(expr) => {
                let val = self.eval(expr)?;
                match val {
                    Value::Scalar(s) => {
                        if s < 0.0 || s.fract() != 0.0 {
                            return Err("Factorial requires a non-negative integer".to_string());
                        }
                        let n = s as u64;
                        let res = (1..=n).fold(1.0, |acc, x| acc * (x as f64));
                        Ok(Value::Scalar(res))
                    }
                    _ => Err("Factorial requires a scalar".to_string()),
                }
            }
            Node::Matrix(rows) => {
                if rows.is_empty() {
                    return Err("Empty matrix".to_string());
                }
                let num_rows = rows.len();
                let num_cols = rows[0].len();
                let mut data = Vec::new();

                for r in 0..num_rows {
                    if rows[r].len() != num_cols {
                        return Err("Matrix rows must have equal lengths".to_string());
                    }
                    for c in 0..num_cols {
                        let val = self.eval(&rows[r][c])?;
                        match val {
                            Value::Scalar(s) => data.push(s),
                            _ => return Err("Matrix elements must be scalars".to_string()),
                        }
                    }
                }
                // nalgebra DMatrix constructor takes data in column-major or row-major
                let mat = DMatrix::from_row_slice(num_rows, num_cols, &data);
                Ok(Value::Matrix(mat))
            }
            Node::Vector(elements) => {
                let mut data = Vec::new();
                for elem in elements {
                    let val = self.eval(elem)?;
                    match val {
                        Value::Scalar(s) => data.push(s),
                        _ => return Err("Vector elements must be scalars".to_string()),
                    }
                }
                Ok(Value::Vector(DVector::from_vec(data)))
            }
        }
    }

    fn eval_binary_op(&self, op: char, l: &Value, r: &Value) -> Result<Value, String> {
        match (l, r) {
            (Value::Scalar(a), Value::Scalar(b)) => match op {
                '+' => Ok(Value::Scalar(a + b)),
                '-' => Ok(Value::Scalar(a - b)),
                '*' => Ok(Value::Scalar(a * b)),
                '/' => {
                    if *b == 0.0 {
                        Err("Divide by zero".to_string())
                    } else {
                        Ok(Value::Scalar(a / b))
                    }
                }
                '%' => Ok(Value::Scalar(a % b)),
                '^' => Ok(Value::Scalar(a.powf(*b))),
                _ => Err(format!("Unknown binary operator: '{}'", op)),
            },
            (Value::Matrix(m1), Value::Matrix(m2)) => match op {
                '+' => {
                    if m1.shape() != m2.shape() {
                        Err("Matrix dimensions must match for addition".to_string())
                    } else {
                        Ok(Value::Matrix(m1 + m2))
                    }
                }
                '-' => {
                    if m1.shape() != m2.shape() {
                        Err("Matrix dimensions must match for subtraction".to_string())
                    } else {
                        Ok(Value::Matrix(m1 - m2))
                    }
                }
                '*' => {
                    if m1.ncols() != m2.nrows() {
                        Err("Inner matrix dimensions must match for multiplication".to_string())
                    } else {
                        Ok(Value::Matrix(m1 * m2))
                    }
                }
                _ => Err(format!("Unsupported matrix operation: '{}'", op)),
            },
            (Value::Matrix(m), Value::Scalar(s)) | (Value::Scalar(s), Value::Matrix(m)) => match op {
                '*' => Ok(Value::Matrix(m * *s)),
                _ => Err(format!("Unsupported scalar-matrix operation: '{}'", op)),
            },
            (Value::Vector(v1), Value::Vector(v2)) => match op {
                '+' => {
                    if v1.len() != v2.len() {
                        Err("Vector dimensions must match for addition".to_string())
                    } else {
                        Ok(Value::Vector(v1 + v2))
                    }
                }
                '-' => {
                    if v1.len() != v2.len() {
                        Err("Vector dimensions must match for subtraction".to_string())
                    } else {
                        Ok(Value::Vector(v1 - v2))
                    }
                }
                _ => Err(format!("Unsupported vector operation: '{}'", op)),
            },
            _ => Err("Incompatible types for binary operation".to_string()),
        }
    }

    fn eval_function(&self, name: &str, args: &[Node]) -> Result<Value, String> {
        let fn_name = name.to_lowercase();

        // Check single-argument functions
        if args.len() == 1 {
            let val = self.eval(&args[0])?;
            match fn_name.as_str() {
                "det" => return linear_algebra::eval_det(&val),
                "inv" => return linear_algebra::eval_inv(&val),
                "eigenval" => return linear_algebra::eval_eigenval(&val),
                "norm" => return linear_algebra::eval_norm(&val),
                _ => {}
            }

            if let Value::Scalar(s) = val {
                let rad = match self.angle_unit {
                    AngleUnit::Deg => s.to_radians(),
                    AngleUnit::Rad => s,
                };

                let res = match fn_name.as_str() {
                    "sin" => rad.sin(),
                    "cos" => rad.cos(),
                    "tan" => rad.tan(),
                    "asin" => {
                        let a = s.asin();
                        match self.angle_unit {
                            AngleUnit::Deg => a.to_degrees(),
                            AngleUnit::Rad => a,
                        }
                    }
                    "acos" => {
                        let a = s.acos();
                        match self.angle_unit {
                            AngleUnit::Deg => a.to_degrees(),
                            AngleUnit::Rad => a,
                        }
                    }
                    "atan" => {
                        let a = s.atan();
                        match self.angle_unit {
                            AngleUnit::Deg => a.to_degrees(),
                            AngleUnit::Rad => a,
                        }
                    }
                    "sinh" => s.sinh(),
                    "cosh" => s.cosh(),
                    "tanh" => s.tanh(),
                    "ln" => s.ln(),
                    "log" | "log10" => s.log10(),
                    "log2" => s.log2(),
                    "sqrt" => s.sqrt(),
                    "abs" => s.abs(),
                    _ => return Err(format!("Unknown single-argument function: '{}'", name)),
                };
                return Ok(Value::Scalar(res));
            }
        }

        // Two argument functions
        if args.len() == 2 {
            let arg1 = self.eval(&args[0])?;
            let arg2 = self.eval(&args[1])?;

            match fn_name.as_str() {
                "dot" => return linear_algebra::eval_dot(&arg1, &arg2),
                "cross" => return linear_algebra::eval_cross(&arg1, &arg2),
                "root" => {
                    if let (Value::Scalar(n), Value::Scalar(x)) = (arg1, arg2) {
                        return Ok(Value::Scalar(x.powf(1.0 / n)));
                    }
                }
                "ncr" => {
                    if let (Value::Scalar(n), Value::Scalar(r)) = (arg1, arg2) {
                        let n_u = n as u64;
                        let r_u = r as u64;
                        if r_u > n_u {
                            return Ok(Value::Scalar(0.0));
                        }
                        let mut num = 1.0;
                        let mut den = 1.0;
                        for i in 1..=r_u {
                            num *= (n_u - r_u + i) as f64;
                            den *= i as f64;
                        }
                        return Ok(Value::Scalar(num / den));
                    }
                }
                "npr" => {
                    if let (Value::Scalar(n), Value::Scalar(r)) = (arg1, arg2) {
                        let n_u = n as u64;
                        let r_u = r as u64;
                        if r_u > n_u {
                            return Ok(Value::Scalar(0.0));
                        }
                        let mut num = 1.0;
                        for i in (n_u - r_u + 1)..=n_u {
                            num *= i as f64;
                        }
                        return Ok(Value::Scalar(num));
                    }
                }
                _ => {}
            }
        }

        Err(format!("Invalid function call: '{}'", name))
    }
}
