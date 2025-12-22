use crate::ast::Expression;
use derive_more::{Display, From};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From)]
#[display("{};", _0)]
pub struct ExpressionStatement(pub Expression);

impl ExpressionStatement {
    pub fn expression(&self) -> &Expression {
        &self.0
    }
}
