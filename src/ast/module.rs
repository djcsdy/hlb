use crate::ast::Declaration;
use derive_more::From;
use itertools::Itertools;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From)]
pub struct Module(pub Vec<Declaration>);

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0.iter().format("\n\n"))
    }
}
