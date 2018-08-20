use ::parser::*;

named!(pub parse_struct<NomSpan, AstItem>,
    do_parse!(
        // zero or more whitespaces, including line endings and tabs
        ws0 >>
        // parse the `struct` keyword
        from: tag!("struct") >>
        // after parsing the keyword, return an error if the parser fails
        data: return_error!(do_parse!(
            // one or more whitespaces, including line endings and tabs
            ws1 >>
            // return `ParseErrorKind::MissingStructName` if the parser fails
            struct_name: add_return_error!(ErrorKind::Custom(ParseErrorKind::InvalidStructName as u32), parse_identifier) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            // return `ParseErrorKind::MissingOpenCurlyBraces` if none given.
            add_return_error!(ErrorKind::Custom(ParseErrorKind::MissingOpenCurlyBraces as u32), tag!("{")) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            member: separated_list!(tag!(","), parse_struct_member) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            opt!(tag!(",")) >>
            // zero or more whitespaces, including line endings and tabs
            ws0 >>
            // return `ParseErrorKind::MissingClosingCurlyBraces` if none given.
            to: add_return_error!(ErrorKind::Custom(ParseErrorKind::MissingClosingCurlyBraces as u32), tag!("}")) >>
            ((struct_name, member, to))
        )) >>
        (AstItem::Struct(StructDeclaration{
            span: Span::from_to(Span::from_nom_span(&from), Span::from_nom_span(&data.2)),
            struct_name: data.0,
            struct_member: data.1,
        }))
    )
);

named!(pub parse_struct_member<NomSpan, StructMemberDeclaration>,
    do_parse!(
        ws0 >>
        struct_member_name: parse_identifier >>
        ws0 >>
        tag!(":") >>
        ws0 >>
        struct_member_type_name: parse_type_identifier >>
        ws0 >>
        (StructMemberDeclaration{
            span: Span::from_to(struct_member_name.get_span(), struct_member_type_name.get_span()),
            struct_member_name: struct_member_name,
            struct_member_type_name: struct_member_type_name,
        })
    )
);
