use crate::ast::Identifier;
use derive_more::with_trait::IsVariant;
use derive_more::{Display, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, IsVariant, Unwrap, TryUnwrap,
)]
pub enum Kind {
    #[display("none")]
    None,
    #[display("string")]
    String,
    #[display("int")]
    Int,
    #[display("bool")]
    Bool,
    #[display("fs")]
    Filesystem,
    #[display("pipeline")]
    Pipeline,
    #[display("option")]
    Options,
    #[display("option::{}", _0)]
    Option(Identifier),
}
