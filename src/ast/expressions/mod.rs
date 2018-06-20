use ::ast::*;

pub mod call_expression;
pub mod field_accessor_expression;
pub mod index_accessor_expression;
pub mod infix_expression;
pub mod literal_expression;
pub mod struct_field_initializer_expression;
pub mod struct_instantiation_expression;
pub mod variable_expression;

pub use self::call_expression::*;
pub use self::field_accessor_expression::*;
pub use self::index_accessor_expression::*;
pub use self::infix_expression::*;
pub use self::literal_expression::*;
pub use self::struct_field_initializer_expression::*;
pub use self::struct_instantiation_expression::*;
pub use self::variable_expression::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Infix(InfixExpression),
    Literal(LiteralExpression),
    Call(CallExpression),
    StructInstantiation(StructInstantiationExpression),
    FieldAccessor(FieldAccessorExpression),
    IndexAccessor(IndexAccesorExpression),
    Variable(VariableExpression),
}

impl Spanned for Expression {
    fn get_span(&self) -> Span {
        match *self {
            Expression::Infix(ref expression) => expression.span,
            Expression::Literal(ref expression) => expression.span,
            Expression::Call(ref expression) => expression.span,
            Expression::StructInstantiation(ref expression) => expression.span,
            Expression::FieldAccessor(ref expression) => expression.span,
            Expression::IndexAccessor(ref expression) => expression.span,
            Expression::Variable(ref expression) => expression.span,
        }
    }
}
