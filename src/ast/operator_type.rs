#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OperatorType {
    Equality,
    Plus,
    Minus,
    Multiply,
    Divide,
}
