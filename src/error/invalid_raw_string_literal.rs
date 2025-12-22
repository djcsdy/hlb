use thiserror::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Error)]
#[error("Invalid raw string literal: {0}")]
pub struct InvalidRawStringLiteral(pub String);
