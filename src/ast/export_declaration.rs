use crate::ast::Identifier;
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("export {}", name)]
pub struct ExportDeclaration {
    pub name: Identifier,
}
