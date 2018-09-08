use nom::types::CompleteStr;
use nom::{Err, ErrorKind, Context};
use nom_locate::LocatedSpan;
use ::ast::*;
use ::error::*;

mod expressions;

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
pub use self::expressions::*;

pub type NomSpan<'a> = LocatedSpan<CompleteStr<'a>>;

named!(parse_number<NomSpan, NomSpan>,
    recognize!(
        do_parse!(
            many1!(one_of!("0123456789")) >>
            ()
        )
    )
);

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

named!(parse<NomSpan, Ast>,
    do_parse!(
        ws0 >>
        ast: many0!(
            alt!(
                parse_struct |
                parse_function
            )
        ) >>
        ws0 >>
        return_error!(add_return_error!(ErrorKind::Custom(ParseErrorKind::InvalidTopLevelItem as u32), eof!())) >>
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
        Err(Err::Incomplete(_needed)) => {
            Err(ParseError::new(Span::new(0, 0, 0, 0), ParseErrorKind::MissingError))
        },
        Err(Err::Error(error)) | Err(Err::Failure(error)) => {
            match error {
                Context::Code(span, ErrorKind::Custom(error_number)) => {
                    return Err(ParseError::new(
                        Span::from_nom_span(&span),
                        ParseErrorKind::from(error_number)
                    ));
                }

                Context::Code(span, _error_kind) => {
                    println!("{:#?}", _error_kind);
                    return Err(ParseError::new(
                        Span::from_nom_span(&span),
                        ParseErrorKind::MissingError,
                    ));
                }
            }
        },
    }
}
