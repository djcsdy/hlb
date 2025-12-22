use crate::ast::StringLiteral;
use crate::ast::raw_string_literal::RawStringLiteral;
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
pub enum BasicLiteral {
    Numeric(u64),
    Boolean(bool),
    String(StringLiteral),
    RawString(RawStringLiteral),
}

impl From<&str> for BasicLiteral {
    fn from(value: &str) -> Self {
        StringLiteral::from(value).into()
    }
}

impl From<String> for BasicLiteral {
    fn from(value: String) -> Self {
        StringLiteral::from(value).into()
    }
}
