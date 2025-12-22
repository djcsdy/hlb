use crate::ast::StringFragment;
use derive_more::AsRef;
use itertools::Itertools;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, AsRef)]
#[as_ref(forward)]
pub struct StringLiteral(pub Vec<StringFragment>);

impl StringLiteral {
    pub fn fragments(&self) -> &Vec<StringFragment> {
        &self.0
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.0.iter().format(""))
    }
}

impl From<&str> for StringLiteral {
    fn from(value: &str) -> Self {
        Self(vec![StringFragment::from(value)])
    }
}

impl From<String> for StringLiteral {
    fn from(value: String) -> Self {
        Self(vec![StringFragment::from(value)])
    }
}
