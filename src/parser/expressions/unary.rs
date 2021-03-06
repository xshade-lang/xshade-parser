use ::parser::*;

named!(pub parse_unary<NomSpan, Expression>,
    do_parse!(
        prefix: parse_unary_prefix >>
        primary: parse_primary >>
        postfix: parse_unary_postfix >>
    (fold_unary(primary, prefix, postfix)))
);

fn fold_unary(mut primary: Expression, mut prefix: Vec<(Span, UnaryOperatorType)>, mut postfix: Vec<UnaryExpressionData>) -> Expression {
    postfix.reverse();
    while let Some(unary_expression_data) = postfix.pop() {
        primary = match unary_expression_data {
            UnaryExpressionData::Field(span, identifier) => Expression::FieldAccessor(FieldAccessorExpression {
                span: Span::from_to(primary.get_span(), span),
                accessee_expression: Box::new(primary),
                field_name: identifier,
            }),
            UnaryExpressionData::Index(span, expression) => Expression::IndexAccessor(IndexAccesorExpression {
                span: Span::from_to(primary.get_span(), span),
                indexee_expression: Box::new(primary),
                index_expression: Box::new(expression),
            }),
        }
    }

    while let Some((span, operator)) = prefix.pop() {
        primary = Expression::Unary(UnaryExpression {
            span: Span::from_to(span, primary.get_span()),
            operator: operator,
            expression: Box::new(primary),
        });
    }

    primary
}

named!(parse_unary_prefix<NomSpan, Vec<(Span, UnaryOperatorType)>>,
    many0!(
        do_parse!(
            operator: alt!(
                tag!("-") |
                tag!("!") |
                tag!("~")
            ) >>
            ((Span::from_nom_span(&operator), parse_unary_operator_type(&operator)))
        )
    )
);

named!(parse_unary_postfix<NomSpan, Vec<UnaryExpressionData>>,
    many0!(alt!(
        parse_indexing |
        parse_field
    ))
);

#[derive(Debug)]
enum UnaryExpressionData {
    Index(Span, Expression),
    Field(Span, String),
}

named!(parse_field<NomSpan, UnaryExpressionData>,
    do_parse!(
        ws0 >>
        dot: tag!(".") >>
        accessor: recognize!(
            do_parse!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                ()
            )
        ) >>
        ws0 >>
    (UnaryExpressionData::Field(Span::from_to(Span::from_nom_span(&dot), Span::from_nom_span(&accessor)), accessor.fragment.to_string())))
);

named!(parse_indexing<NomSpan, UnaryExpressionData>,
    do_parse!(
        ws0 >>
        open_tag: tag!("[") >>
        ws0 >>
        indexer: parse_expression >>
        ws0 >>
        close_tag: tag!("]") >>
    (UnaryExpressionData::Index(Span::from_to(Span::from_nom_span(&open_tag), Span::from_nom_span(&close_tag)), indexer)))
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse_expression(NomSpan::new(CompleteStr("-b[a].c"))).unwrap();
        assert!(result.0.fragment.len() == 0);
    }
}
