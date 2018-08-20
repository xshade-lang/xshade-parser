use nom::types::CompleteStr;
use nom::{Err, ErrorKind, Context, Needed};
use nom_locate::LocatedSpan;
use ::ast::*;
use ::error::*;

mod ws;
mod identifier;
mod structs;
mod functions;
mod types;

use self::ws::*;
pub use self::identifier::*;
pub use self::structs::*;
pub use self::functions::*;
pub use self::types::*;

pub type NomSpan<'a> = LocatedSpan<CompleteStr<'a>>;

named!(parse_number<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            many1!(one_of!("0123456789")) >>
            ()
        )
    )
);

fn parse_float_literal(before: NomSpan, after: NomSpan) -> Expression {
    let mut a: String = before.fragment.to_string();
    let b: String = after.fragment.to_string();
    a.push_str(".");
    a.push_str(&b);
    Expression::Literal(LiteralExpression {
        span: Span::from_to(Span::from_nom_span(&before), Span::from_nom_span(&after)),
        value: a,
        literal_expression_type: LiteralType::Float,
    })
}

fn parse_int_literal(parts: NomSpan) -> Expression {
    let string: String = parts.fragment.to_string();
    Expression::Literal(LiteralExpression {
        span: Span::from_nom_span(&parts),
        value: string,
        literal_expression_type: LiteralType::Int,
    })
}

named!(parse_operator<NomSpan, OperatorType>,
    alt!(
        do_parse!(char!('+') >> (OperatorType::Plus)) |
        do_parse!(char!('-') >> (OperatorType::Minus)) |
        do_parse!(char!('*') >> (OperatorType::Multiply)) |
        do_parse!(char!('/') >> (OperatorType::Divide))
    )
);

named!(parse_local_statement<NomSpan, Statement>,
    do_parse!(
        from: ws!(tag!("let")) >>
        symbol_name: parse_identifier >>
        ws!(tag!("=")) >>
        expression: parse_expression >>
        to: ws!(tag!(";")) >>
        (Statement::Local(
            LocalStatement{
                span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
                symbol_name: symbol_name,
                expression: expression,
            }
        ))
    )
);

named!(parse_return_statement<NomSpan, Statement>,
    do_parse!(
        from: ws!(tag!("return")) >>
        expression: parse_expression >>
        to: ws!(tag!(";")) >>
        (Statement::Return(ReturnStatement{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            expression: expression,
        }))
    )
);

named!(parse_expression_statement<NomSpan, Statement>,
    do_parse!(
        expression: parse_expression >>
        semicolon: ws!(tag!(";")) >>
        (Statement::Expression(
            ExpressionStatement {
                span: Span::from_to(expression.get_span(), Span::from_nom_span(&semicolon)),
                expression,
            }
        ))
    )
);

named!(parse_block_statements<NomSpan, Vec<Statement>>,
    many0!(
        ws!(
            alt!(
                parse_local_statement |
                parse_return_statement |
                parse_expression_statement
            )
        )
    )
);

named!(parse_block_declaration<NomSpan, BlockDeclaration>,
    do_parse!(
        from: ws!(tag!("{")) >>
        statements: parse_block_statements >>
        to: ws!(tag!("}")) >>
        (BlockDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            statements: statements,
        })
    )
);

named!(parse_infix_expression<NomSpan, Expression>,
    do_parse!(
        left: parse_expression_no_left_recursion >>
        operator: parse_operator >>
        right: parse_expression >>
        (Expression::Infix(InfixExpression{
            span: Span::from_to(left.get_span(), right.get_span()),
            operator: operator,
            left_hand: Box::new(left),
            right_hand: Box::new(right),
        }))
    )
);

named!(parse_variable_expression<NomSpan, Expression>,
    do_parse!(
        variable_name: parse_identifier >>
        (Expression::Variable(VariableExpression{
            span: variable_name.span.clone(),
            variable_name: variable_name,
        }))
    )
);

named!(parse_call<NomSpan, CallExpression>,
    do_parse!(
        function_name: parse_identifier >>
        ws!(tag!("(")) >>
        arguments: ws!(separated_list!(tag!(","), parse_expression)) >>
        to: ws!(tag!(")")) >>
        (CallExpression {
            span: Span::from_to(function_name.span, Span::from_nom_span(&to)),
            function_name: function_name,
            arguments: arguments,
        })
    )
);

named!(parse_call_expression<NomSpan, Expression>,
    do_parse!(
        call: parse_call >>
        (Expression::Call(call))
    )
);

// TODO nested accessor expressions like `a.b.c`
named!(parse_field_accessor_expression<NomSpan, Expression>,
    do_parse!(
        variable_name: parse_identifier >>
        ws!(tag!(".")) >>
        field_name: parse_identifier >>
        (Expression::FieldAccessor(FieldAccessorExpression{
            span: Span::from_to(variable_name.span, field_name.span),
            variable_name: variable_name,
            field_name: field_name,
        }))
    )
);


named!(parse_float_literal_expression<NomSpan, Expression>,
    do_parse!(
        before: ws!(parse_number) >>
        ws!(tag!(".")) >>
        after: ws!(parse_number) >>
        (parse_float_literal(before, after))
    )
);

named!(parse_int_literal_expression<NomSpan, Expression>,
    do_parse!(
        numbers: ws!(parse_number) >>
        (parse_int_literal(numbers))
    )
);

// TODO more literals
named!(parse_literal_expression<NomSpan, Expression>,
    alt!(
        parse_float_literal_expression |
        parse_int_literal_expression
    )
);

named!(parse_struct_instantiation_field_initializer<NomSpan, StructFieldInitializerExpression>,
    do_parse!(
        struct_field_name: parse_identifier >>
        ws!(tag!(":")) >>
        initializer: parse_expression >>
        (StructFieldInitializerExpression{
            span: Span::from_to(struct_field_name.span, initializer.get_span()),
            struct_field_name: struct_field_name,
            initializer: Box::new(initializer),
        })
    )
);

named!(parse_struct_instantiation<NomSpan, Expression>,
    do_parse!(
        struct_type_name: parse_type_identifier >>
        ws!(tag!("{")) >>
        struct_field_initializer: ws!(separated_list!(tag!(","), parse_struct_instantiation_field_initializer)) >>
        opt!(ws!(tag!(","))) >>
        to: ws!(tag!("}")) >>
        (Expression::StructInstantiation(StructInstantiationExpression{
            span: Span::from_to(struct_type_name.get_span(), Span::from_nom_span(&to)),
            struct_type_name: struct_type_name,
            struct_field_initializer: struct_field_initializer,
        }))
    )
);

// TODO precedence
// TODO parentheses
named!(parse_expression<NomSpan, Expression>,
    alt!(
        parse_infix_expression |
        parse_struct_instantiation |
        parse_literal_expression |
        parse_field_accessor_expression |
        parse_call_expression |
        parse_variable_expression
    )
);
named!(parse_expression_no_left_recursion<NomSpan, Expression>,
    alt!(
        parse_struct_instantiation |
        parse_literal_expression |
        parse_field_accessor_expression |
        parse_call_expression |
        parse_variable_expression
    )
);

named!(parse<NomSpan, Ast>,
    do_parse!(
        ast: many0!(
            alt!(
                parse_struct |
                parse_function
            )
        ) >>
        ws0 >>
        eof!() >>
        (ast)
    )
);

/// Parses a string into an untyped AST
/// 
/// # Example
/// ```
/// # extern crate xshade_parser;
/// # use xshade_parser::parse_str;
/// let ast = parse_str("struct Foo { bar: f32, }").unwrap();
/// ```
pub fn parse_str(program: &str) -> ParseResult<Ast> {
    let input = NomSpan::new(CompleteStr(program));
    let output = parse(input);

    match output {
        Ok((remaining, ast)) => {
            if remaining.fragment.len() > 0 {
                unimplemented!("handle remaining")
            }

            Ok(ast)
        },
        Err(Err::Incomplete(needed)) => {
            unimplemented!("handle incomplete")
        },
        Err(Err::Error(error)) | Err(Err::Failure(error)) => {
            match error {
                Context::Code(a, ErrorKind::Custom(error_number)) => {
                    return Err(ParseError::new(
                        Span::from_nom_span(&a),
                        ParseErrorKind::from(error_number)
                    ));
                }

                _ => unimplemented!("handle error")
            }
        },
    }
}
