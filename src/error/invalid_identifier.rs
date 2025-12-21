use thiserror::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Error)]
#[error("Invalid Identifier: {0}")]
pub struct InvalidIdentifier(pub String);
