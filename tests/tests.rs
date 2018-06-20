extern crate xshade_parser;

use xshade_parser::*;

#[test]
fn it_parses_a_struct() {
    let source = include_str!("struct.xs");
    let result = parse_str(source).unwrap();

    assert_eq!(result, vec![AstItem::Struct(
        StructDeclaration {
            span: Span::new(0, 44, 1, 1),
            struct_name: Identifier::new("VertexInput", Span::new(7, 11, 1, 8)),
            struct_member: vec![
                StructMemberDeclaration {
                    span: Span::new(26, 14, 2, 5),
                    struct_member_name: Identifier::new("position", Span::new(26, 8, 2, 5)),
                    struct_member_type_name: Identifier::new("vec3", Span::new(36, 4, 2, 15)),
                }
            ]
        }
    )]);
}
