use tui_calculator::config::AngleUnit;
use tui_calculator::eval::{Evaluator, Value};
use tui_calculator::parser::parse_expression;
use std::collections::HashMap;

fn eval_str(expr: &str, unit: AngleUnit) -> Result<Value, String> {
    let vars = HashMap::new();
    let ast = parse_expression(expr)?;
    let evaluator = Evaluator::new(&vars, unit);
    evaluator.eval(&ast)
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(eval_str("2 + 3 * 4", AngleUnit::Deg).unwrap(), Value::Scalar(14.0));
    assert_eq!(eval_str("10 - 6 / 2", AngleUnit::Deg).unwrap(), Value::Scalar(7.0));
    assert_eq!(eval_str("2^3", AngleUnit::Deg).unwrap(), Value::Scalar(8.0));
    assert_eq!(eval_str("5!", AngleUnit::Deg).unwrap(), Value::Scalar(120.0));
}

#[test]
fn test_latex_expression() {
    assert_eq!(eval_str("\\frac{10}{2}", AngleUnit::Deg).unwrap(), Value::Scalar(5.0));
    assert_eq!(eval_str("\\sqrt{16}", AngleUnit::Deg).unwrap(), Value::Scalar(4.0));
    if let Value::Scalar(s) = eval_str("\\sin(30)", AngleUnit::Deg).unwrap() {
        assert!((s - 0.5).abs() < 1e-6);
    } else {
        panic!("Expected scalar result");
    }
}

#[test]
fn test_trig_deg_vs_rad() {
    let deg_res = eval_str("sin(90)", AngleUnit::Deg).unwrap();
    let rad_res = eval_str("sin(pi / 2)", AngleUnit::Rad).unwrap();
    
    if let (Value::Scalar(d), Value::Scalar(r)) = (deg_res, rad_res) {
        assert!((d - 1.0).abs() < 1e-6);
        assert!((r - 1.0).abs() < 1e-6);
    } else {
        panic!("Expected scalar result");
    }
}

#[test]
fn test_matrix_operations() {
    // Determinant of [[1, 2], [3, 4]] -> (1*4 - 2*3) = -2
    let det_res = eval_str("det([[1, 2], [3, 4]])", AngleUnit::Deg).unwrap();
    assert_eq!(det_res, Value::Scalar(-2.0));
}

#[test]
fn test_divide_by_zero() {
    assert!(eval_str("10 / 0", AngleUnit::Deg).is_err());
}

#[test]
fn test_smart_fraction_and_sqrt() {
    let mut app = tui_calculator::app::App::new();
    app.insert_str("25");
    app.insert_fraction();
    assert_eq!(app.input, "\\frac{25}{}");

    app.backspace();
    assert_eq!(app.input, "25");

    app.clear_input();
    app.insert_sqrt();
    app.insert_str("16)");
    assert_eq!(app.input, "√(16)");
}
