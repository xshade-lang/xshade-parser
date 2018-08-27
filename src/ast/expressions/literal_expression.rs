use ::ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiteralExpression {
    pub span: Span,
    pub value: String,
    pub literal_expression_type: LiteralType,
}

impl Spanned for LiteralExpression {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl Execute for LiteralExpression {
    fn execute(&self) -> Option<i32> {
        match self.literal_expression_type {
            LiteralType::Int => Some(self.value.parse::<i32>().unwrap()),
            _ => None,
        }
    }
}
