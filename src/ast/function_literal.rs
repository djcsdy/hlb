use crate::ast::{BlockStatement, Kind};
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("{} {}", kind, body)]
pub struct FunctionLiteral {
    pub kind: Kind,
    pub body: BlockStatement,
}
