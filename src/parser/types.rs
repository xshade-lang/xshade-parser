use ::parser::*;
use ::ast::type_identifier::TypeIdentifier;

named!(pub parse_type_identifier(NomSpan) -> TypeIdentifier,
    alt!(
        parse_named_tuple_type_identifier |
        parse_tuple_type_identifier |
        parse_single_type_identifier
    )
);

named!(parse_named_tuple_type_identifier(NomSpan) -> TypeIdentifier,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the opening tag of the tuple's member list
        from: tag!("(") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the tuple's member list
        member: separated_list!(tag!(","), parse_named_tuple_type_identifier_member) >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the closing tag of the tuple's member list
        to: tag!(")") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        (TypeIdentifier::NamedTuple(NamedTupleTypeIdentifier {
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            member: member,
        }))
    )
);

named!(parse_named_tuple_type_identifier_member(NomSpan) -> NamedTupleTypeMember,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the name of the member
        name: parse_identifier >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the separator between name and type
        tag!(":") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the type name of the member
        type_identifier: parse_type_identifier >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        (NamedTupleTypeMember {
            span: Span::from_to(name.get_span(), type_identifier.get_span()),
            name: name.name,
            type_identifier: type_identifier,
        })
    )
);

named!(parse_tuple_type_identifier(NomSpan) -> TypeIdentifier,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the opening tag of the tuple's member list
        from: tag!("(") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the tuple's member list
        member: separated_list!(tag!(","), parse_type_identifier) >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the closing tag of the tuple's member list
        to: tag!(")") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        (TypeIdentifier::Tuple(TupleTypeIdentifier {
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&to)),
            member: member,
        }))
    )
);

named!(parse_single_type_identifier(NomSpan) -> TypeIdentifier,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        span: recognize!(
            do_parse!(
                one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") >>
                many0!(one_of!("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")) >>
                ()
            )
        ) >>
        arguments: opt!(
            do_parse!(
                tag!("<") >>
                arguments: separated_list!(tag!(","), parse_type_identifier) >>
                tag!(">") >>
            (arguments))
        ) >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        (TypeIdentifier::Single(SingleTypeIdentifier {
            span: Span::from_nom_span(&span),
            name: span.fragment.to_string(),
            arguments: arguments.unwrap_or(vec![]),
        }))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_named_tuple_type() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("(a: A, b: B)"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }

    #[test]
    fn it_parses_a_tuple_type() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("(A, B)"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }

    #[test]
    fn it_parses_a_single_type() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("A"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }

    #[test]
    fn it_parses_a_single_type_with_type_arguments() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("A<B>"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }

    #[test]
    fn it_parses_a_single_type_with_nested_type_arguments() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("A<B<C>>"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }

    #[test]
    fn it_parses_a_ridiculous_type() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("(a: A<B<C>>, b: (a: A, b: B), c: (A, B<(A<B>, C)>))"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }
    
    #[test]
    fn it_parses_a_sane_type() {
        let result = parse_type_identifier(NomSpan::new(CompleteStr("List<(number: int, condition: bool)>"))).unwrap();
        assert!(result.0.fragment.len() == 0, "parser has no remaining input");
    }
}
