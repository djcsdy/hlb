use crate::ast::{BasicLiteral, CallExpression, FunctionLiteral, RawStringLiteral, StringLiteral};
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
pub enum Expression {
    FunctionLiteral(FunctionLiteral),
    BasicLiteral(BasicLiteral),
    CallExpression(CallExpression),
}

impl From<u64> for Expression {
    fn from(value: u64) -> Self {
        Self::BasicLiteral(BasicLiteral::from(value))
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        Self::BasicLiteral(BasicLiteral::from(value))
    }
}

impl From<StringLiteral> for Expression {
    fn from(value: StringLiteral) -> Self {
        Self::BasicLiteral(BasicLiteral::from(value))
    }
}

impl From<RawStringLiteral> for Expression {
    fn from(value: RawStringLiteral) -> Self {
        Self::BasicLiteral(BasicLiteral::from(value))
    }
}
