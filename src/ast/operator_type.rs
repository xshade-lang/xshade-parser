#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
}
