use crate::ast::{CallStatement, ExpressionStatement};
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
#[display("{}\n", _0)]
pub enum Statement {
    Call(CallStatement),
    Expression(ExpressionStatement),
}
