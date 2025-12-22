use crate::ast::{ExpressionList, Identifier};
use derive_more::Display;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display)]
#[display("{}{}", name, arguments)]
pub struct CallExpression {
    pub name: Identifier,
    pub arguments: ExpressionList,
}
