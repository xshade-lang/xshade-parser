use ::parser::*;

named!(pub parse_function<NomSpan, AstItem>,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the `fn` keyword
        from: tag!("fn") >>
        // after the function keyword, return an error if parsing fails
        data: return_error!(do_parse!(
            // parse one or more whitespaces, including line endings and tabs
            ws1 >>
            // parse the function name, return an InvalidFunctionName error if it failed
            function_name: add_return_error!(ErrorKind::Custom(ParseErrorKind::InvalidFunctionName as u32), parse_identifier) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            // parse function type arguments
            type_arguments: opt!(do_parse!(
                tag!("<") >>
                type_arguments: separated_list!(tag!(","), parse_identifier) >>
                tag!(">") >>
                (type_arguments)
            )) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            // parse the beginning of the argument list
            add_return_error!(ErrorKind::Custom(ParseErrorKind::MissingArgumentList as u32), tag!("(")) >>
            // parse the argument list
            arguments: separated_list!(tag!(","), parse_function_argument) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            // parse the end of the argument list
            void: tag!(")") >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            return_type_name: opt!(do_parse!(
                // parse the arrow indicating a following return type
                tag!("->") >>
                // zero or more whitespaces, including line endings and tabs
                ws0 >>
                // parse return type
                return_type_name: parse_type_identifier >>
                // zero or more whitespaces, including line endings and tabs
                ws0 >>
                (return_type_name)
            )) >>
            // parse the function body as a block
            block: parse_block_declaration >>
            ((function_name, type_arguments, arguments, return_type_name, block, void))
        )) >>
        (AstItem::Function(FunctionDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), data.4.span),
            function_name: data.0,
            type_arguments: data.1.unwrap_or(vec![]),
            arguments: data.2,
            return_type_name: data.3.unwrap_or(TypeIdentifier::void(Span::from_nom_span(&data.5))),
            block: data.4,
        }))
    )
);

named!(parse_function_argument<NomSpan, FunctionArgumentDeclaration>,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the argument identifier
        argument_name: parse_identifier >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the `:` denoting a following type information
        tag!(":") >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the argument's type identifier
        argument_type_name: parse_type_identifier >>
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        (FunctionArgumentDeclaration{
            span: Span::from_to(argument_name.get_span(), argument_type_name.get_span()),
            argument_name: argument_name,
            argument_type_name: argument_type_name,
        })
    )
);
