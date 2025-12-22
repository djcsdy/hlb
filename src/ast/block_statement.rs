use crate::ast::Statement;
use derive_more::{AsRef, From};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From, AsRef)]
#[as_ref(forward)]
pub struct BlockStatement(pub Vec<Statement>);

impl BlockStatement {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.0
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n{}}}", self.0.iter().format(""))
    }
}
