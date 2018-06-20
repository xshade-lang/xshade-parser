pub mod span;
pub mod identifier;
pub mod block_declaration;
pub mod function_argument_declaration;
pub mod function_declaration;
pub mod literal_type;
pub mod operator_type;
pub mod program_declaration;
pub mod program_stage_declaration;
pub mod struct_declaration;
pub mod struct_member_declaration;
pub mod statements;
pub mod expressions;
pub mod import_declaration;
pub mod export_declaration;

pub use self::span::*;
pub use self::identifier::*;
pub use self::block_declaration::*;
pub use self::function_argument_declaration::*;
pub use self::function_declaration::*;
pub use self::literal_type::*;
pub use self::operator_type::*;
pub use self::program_declaration::*;
pub use self::program_stage_declaration::*;
pub use self::struct_declaration::*;
pub use self::struct_member_declaration::*;
pub use self::statements::*;
pub use self::expressions::*;
pub use self::import_declaration::*;
pub use self::export_declaration::*;

pub trait Spanned {
    fn get_span(&self) -> Span;
}

pub type Ast = Vec<AstItem>;

#[derive(Debug, Eq, PartialEq)]
pub enum AstItem {
    Import(ImportDeclaration),
    Export(ExportDeclaration),
    Struct(StructDeclaration),
    Program(ProgramDeclaration),
    Function(FunctionDeclaration),
    Block(BlockDeclaration),
}

impl Spanned for AstItem {
    fn get_span(&self) -> Span {
        match *self {
            AstItem::Import(ref item) => item.span,
            AstItem::Export(ref item) => item.span,
            AstItem::Struct(ref item) => item.span,
            AstItem::Program(ref item) => item.span,
            AstItem::Function(ref item) => item.span,
            AstItem::Block(ref item) => item.span,
        }
    }
}
