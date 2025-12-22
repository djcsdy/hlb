use crate::ast::{BindList, Identifier};
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
#[display("as {}", _0)]
pub enum BindClause {
    Identifier(Identifier),
    Binds(BindList),
}
