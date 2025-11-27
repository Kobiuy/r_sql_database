use crate::database::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Eq, // =
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=
}

pub struct Condition {
    pub field_name: String,
    pub op: Op,
    pub value: Value,
}

impl Op {
    pub fn as_str(s: &str) -> Option<Self> {
        match s {
            "=" => Some(Op::Eq),
            "!=" => Some(Op::Ne),
            "<" => Some(Op::Lt),
            "<=" => Some(Op::Le),
            ">" => Some(Op::Gt),
            ">=" => Some(Op::Ge),
            _ => None,
        }
    }
    pub fn get_options() -> Vec<String> {
        ["=", "!=", "<", "<=", ">", ">="]
            .iter()
            .map(|v| v.to_string())
            .collect()
    }

    pub fn cmp(&self, v1: &Value, v2: &Value) -> bool {
        match self {
            Op::Eq => v1 == v2,
            Op::Ne => v1 != v2,
            Op::Lt => match (v1, v2) {
                (Value::Int(a), Value::Int(b)) => a < b,
                (Value::Float(a), Value::Float(b)) => a < b,
                (Value::String(a), Value::String(b)) => a < b,
                _ => false, // mismatched types cannot be compared
            },
            Op::Le => match (v1, v2) {
                (Value::Int(a), Value::Int(b)) => a <= b,
                (Value::Float(a), Value::Float(b)) => a <= b,
                (Value::String(a), Value::String(b)) => a <= b,
                _ => false,
            },
            Op::Gt => match (v1, v2) {
                (Value::Int(a), Value::Int(b)) => a > b,
                (Value::Float(a), Value::Float(b)) => a > b,
                (Value::String(a), Value::String(b)) => a > b,
                _ => false,
            },
            Op::Ge => match (v1, v2) {
                (Value::Int(a), Value::Int(b)) => a >= b,
                (Value::Float(a), Value::Float(b)) => a >= b,
                (Value::String(a), Value::String(b)) => a >= b,
                _ => false,
            },
        }
    }
}
impl Condition {
    pub fn new(
        field_name: impl Into<String>,
        op: Op,
        raw_value: &str,
        field_type: &str,
    ) -> Result<Self, String> {
        let value = match Value::from_string(raw_value, field_type) {
            Some(v) => v,
            None => {
                return Err(format!(
                    "Cannot parse value '{}' as type '{}'",
                    raw_value, field_type
                )
                .to_string());
            }
        };

        Ok(Self {
            field_name: field_name.into(),
            op,
            value,
        })
    }
}
