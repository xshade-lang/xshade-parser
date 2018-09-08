use ::ast::*;

pub mod binary_expression;
pub mod call_expression;
pub mod field_accessor_expression;
pub mod index_accessor_expression;
pub mod literal_expression;
pub mod struct_field_initializer_expression;
pub mod struct_instantiation_expression;
pub mod unary_expression;
pub mod variable_expression;

pub use self::binary_expression::*;
pub use self::call_expression::*;
pub use self::field_accessor_expression::*;
pub use self::index_accessor_expression::*;
pub use self::literal_expression::*;
pub use self::struct_field_initializer_expression::*;
pub use self::struct_instantiation_expression::*;
pub use self::unary_expression::*;
pub use self::variable_expression::*;

pub trait Execute {
    fn execute(&self) -> Option<i32>;
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Binary(BinaryExpression),
    Literal(LiteralExpression),
    Call(CallExpression),
    StructInstantiation(StructInstantiationExpression),
    FieldAccessor(FieldAccessorExpression),
    IndexAccessor(IndexAccesorExpression),
    Unary(UnaryExpression),
    Variable(VariableExpression),
}

impl Execute for Expression {
    fn execute(&self) -> Option<i32> {
        match *self {
            Expression::Binary(ref expression) => expression.execute(),
            Expression::Literal(ref expression) => expression.execute(),
            Expression::Call(ref expression) => expression.execute(),
            Expression::StructInstantiation(ref expression) => expression.execute(),
            Expression::FieldAccessor(ref expression) => expression.execute(),
            Expression::IndexAccessor(ref expression) => expression.execute(),
            Expression::Unary(ref expression) => expression.execute(),
            Expression::Variable(ref expression) => expression.execute(),
        }
    }
}

impl Spanned for Expression {
    fn get_span(&self) -> Span {
        match *self {
            Expression::Binary(ref expression) => expression.span,
            Expression::Literal(ref expression) => expression.span,
            Expression::Call(ref expression) => expression.span,
            Expression::StructInstantiation(ref expression) => expression.span,
            Expression::FieldAccessor(ref expression) => expression.span,
            Expression::IndexAccessor(ref expression) => expression.span,
            Expression::Unary(ref expression) => expression.span,
            Expression::Variable(ref expression) => expression.span,
        }
    }
}
