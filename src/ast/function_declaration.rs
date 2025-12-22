use crate::ast::{BlockStatement, FunctionSignature};
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("{} {}", signature, body)]
pub struct FunctionDeclaration {
    pub signature: FunctionSignature,
    pub body: BlockStatement,
}
