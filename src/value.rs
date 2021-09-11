#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(u64),
    Array(Vec<Value>),
}
