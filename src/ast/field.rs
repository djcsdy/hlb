use crate::ast::modifier::Modifier;
use crate::ast::{Identifier, Kind};
use std::fmt;
use std::fmt::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct Field {
    pub modifier: Option<Modifier>,
    pub kind: Kind,
    pub name: Identifier,
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(modifier) = &self.modifier {
            write!(f, "{} ", modifier)?;
        }
        write!(f, "{} {}", self.kind, self.name)
    }
}
