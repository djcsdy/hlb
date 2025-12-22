use crate::ast::Expression;
use derive_more::{AsRef, From};
use itertools::Itertools;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From, AsRef)]
#[as_ref(forward)]
pub struct ExpressionList(pub Vec<Expression>);

impl ExpressionList {
    pub fn expressions(&self) -> &Vec<Expression> {
        &self.0
    }
}

impl Display for ExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.iter().format(", "))
    }
}
