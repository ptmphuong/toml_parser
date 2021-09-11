#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(u64),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_string(&self) -> Option<String> {
        match self {
            Value::String(v) => Some(v.to_string()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }
}
