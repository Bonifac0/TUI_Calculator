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

#[test]
fn test_fraction_arrow_navigation() {
    let mut app = tui_calculator::app::App::new();

    // "1" to the left → grabbed into numerator, cursor lands in denominator
    app.insert_str("1");
    app.insert_fraction();         // numerator="1", cursor: denominator @ 0
    app.insert_str("3");           // denominator = "3"
    assert_eq!(app.input, "\\frac{1}{3}");

    // Right at end of denominator → exit fraction, cursor after it in root
    app.move_cursor_right();
    app.insert_str("4");
    assert_eq!(app.input, "\\frac{1}{3}4");

    // Home → root cursor at 0, Right → enters numerator at start
    app.move_cursor_home();
    app.move_cursor_right();
    app.insert_str("X");
    assert_eq!(app.input, "\\frac{X1}{3}4");

    // Home inside numerator → cursor at 0, Left → exits to root before fraction
    app.move_cursor_home();
    app.move_cursor_left();
    app.insert_str("9");
    assert_eq!(app.input, "9\\frac{X1}{3}4");
}

#[test]
fn test_nested_fraction_vertical_navigation_keeps_visual_x() {
    let mut app = tui_calculator::app::App::new();

    // Build: rac{1}{rac{2}{3}}
    app.insert_str("1");
    app.insert_fraction();
    app.insert_str("2");
    app.insert_fraction();
    app.insert_str("3");
    assert_eq!(app.input, "\\frac{1}{\\frac{2}{3}}");

    // Move to outer numerator near "1"
    app.move_cursor_home();
    app.move_cursor_left();
    app.scroll_up();

    // Down goes through nested fraction into inner numerator near '2'.
    app.scroll_down();
    app.insert_str("X");
    assert_eq!(app.input, "\\frac{1}{\\frac{X2}{3}}");

    // From inner numerator, Up should move out to outer numerator near '1'.
    app.move_cursor_home();
    app.scroll_up();
    app.insert_str("Y");
    assert_eq!(app.input, "\\frac{Y1}{\\frac{X2}{3}}");
}


#[test]
fn test_fraction_consumes_full_number_when_cursor_inside() {
    let mut app = tui_calculator::app::App::new();
    app.insert_str("12345");

    // Move cursor into the middle: between '3' and '4'.
    app.move_cursor_left();
    app.move_cursor_left();

    app.insert_fraction();
    assert_eq!(app.input, "\\frac{12345}{}");

    // Cursor should land in denominator when a token is consumed.
    app.insert_str("2");
    assert_eq!(app.input, "\\frac{12345}{2}");
}
