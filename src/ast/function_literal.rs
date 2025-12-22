use crate::ast::Kind;
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("{} {}", kind, "")] // TODO body
pub struct FunctionLiteral {
    pub kind: Kind,
    pub body: (), // TODO BlockStatement
}
