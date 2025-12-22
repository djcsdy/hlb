use crate::ast::{Expression, Identifier};
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("import {} from {}", name, from)]
pub struct ImportDeclaration {
    pub name: Identifier,
    pub from: Expression,
}
