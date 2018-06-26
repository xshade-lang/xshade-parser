use ::ast::*;

pub mod expression_statement;
pub mod local_statement;
pub mod return_statement;

pub use self::expression_statement::*;
pub use self::local_statement::*;
pub use self::return_statement::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// e.g. a `let` statement
    Local(LocalStatement),

    /// return statement
    Return(ReturnStatement),

    /// statement with only expressions e.g. `my_fn();`
    Expression(ExpressionStatement),
}


impl Spanned for Statement {
    fn get_span(&self) -> Span {
        match *self {
            Statement::Local(ref statement) => statement.span,
            Statement::Return(ref statement) => statement.span,
            Statement::Expression(ref statement) => statement.span,
        }
    }
}
