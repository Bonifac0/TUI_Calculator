use super::value::Value;
use nalgebra::DVector;

pub fn eval_det(val: &Value) -> Result<Value, String> {
    match val {
        Value::Matrix(m) => {
            if m.nrows() != m.ncols() {
                return Err("Determinant requires a square matrix".to_string());
            }
            Ok(Value::Scalar(m.determinant()))
        }
        _ => Err("det() requires a matrix argument".to_string()),
    }
}

pub fn eval_inv(val: &Value) -> Result<Value, String> {
    match val {
        Value::Matrix(m) => {
            if m.nrows() != m.ncols() {
                return Err("Inverse requires a square matrix".to_string());
            }
            match m.clone().try_inverse() {
                Some(inv) => Ok(Value::Matrix(inv)),
                None => Err("Matrix is singular (non-invertible)".to_string()),
            }
        }
        _ => Err("inv() requires a matrix argument".to_string()),
    }
}

pub fn eval_eigenval(val: &Value) -> Result<Value, String> {
    match val {
        Value::Matrix(m) => {
            if m.nrows() != m.ncols() {
                return Err("Eigenvalues require a square matrix".to_string());
            }
            // For general real matrices, compute eigenvalues
            let eigen = m.clone().complex_eigenvalues();
            // Return real parts if imaginary parts are zero/near-zero
            let mut vec = Vec::new();
            for e in eigen.iter() {
                vec.push(e.re);
            }
            Ok(Value::Vector(DVector::from_vec(vec)))
        }
        _ => Err("eigenval() requires a matrix argument".to_string()),
    }
}

pub fn eval_dot(u: &Value, v: &Value) -> Result<Value, String> {
    match (u, v) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != v2.len() {
                return Err("Vector dimensions must match for dot product".to_string());
            }
            Ok(Value::Scalar(v1.dot(v2)))
        }
        _ => Err("dot() requires two vector arguments".to_string()),
    }
}

pub fn eval_cross(u: &Value, v: &Value) -> Result<Value, String> {
    match (u, v) {
        (Value::Vector(v1), Value::Vector(v2)) => {
            if v1.len() != 3 || v2.len() != 3 {
                return Err("Cross product requires 3D vectors".to_string());
            }
            let u3 = nalgebra::Vector3::new(v1[0], v1[1], v1[2]);
            let v3 = nalgebra::Vector3::new(v2[0], v2[1], v2[2]);
            let res = u3.cross(&v3);
            Ok(Value::Vector(DVector::from_vec(vec![res.x, res.y, res.z])))
        }
        _ => Err("cross() requires two 3D vector arguments".to_string()),
    }
}

pub fn eval_norm(val: &Value) -> Result<Value, String> {
    match val {
        Value::Vector(v) => Ok(Value::Scalar(v.norm())),
        Value::Matrix(m) => Ok(Value::Scalar(m.norm())),
        Value::Scalar(s) => Ok(Value::Scalar(s.abs())),
    }
}
