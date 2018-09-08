#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OperatorType {
    Equality,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperatorType {
    Negate,
    Not,
    Complement,
}
