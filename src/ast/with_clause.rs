use crate::ast::Expression;
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("with {}", _0)]
pub struct WithClause(pub Expression);

impl WithClause {
    pub fn expression(&self) -> &Expression {
        &self.0
    }
}
