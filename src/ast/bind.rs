use crate::ast::Identifier;
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("{} {}", source, target)]
pub struct Bind {
    pub source: Identifier,
    pub target: Identifier,
}
