use ast::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeIdentifier {
    Single(SingleTypeIdentifier), // A<T>
    Tuple(TupleTypeIdentifier), // (A<T>, B)
    NamedTuple(NamedTupleTypeIdentifier), // (a: A<T>, b: B)
}

impl TypeIdentifier {
    pub fn void(span: Span) -> TypeIdentifier {
        TypeIdentifier::Tuple(TupleTypeIdentifier {
            span,
            member: vec![],
        })
    }
}

impl Spanned for TypeIdentifier {
    fn get_span(&self) -> Span {
        match self {
            TypeIdentifier::Single(value) => value.span,
            TypeIdentifier::Tuple(value) => value.span,
            TypeIdentifier::NamedTuple(value) => value.span,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SingleTypeIdentifier {
    pub span: Span,
    pub name: String,
    pub arguments: Vec<TypeIdentifier>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TupleTypeIdentifier {
    pub span: Span,
    pub member: Vec<TypeIdentifier>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NamedTupleTypeIdentifier {
    pub span: Span,
    pub member: Vec<NamedTupleTypeMember>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NamedTupleTypeMember {
    pub span: Span,
    pub name: String,
    pub type_identifier: TypeIdentifier,
}
