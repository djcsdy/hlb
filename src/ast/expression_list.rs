use derive_more::{AsRef, From};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, From, AsRef)]
#[as_ref(forward)]
pub struct ExpressionList(pub Vec<()>); // TODO Vec<Expression>

impl ExpressionList {
    pub fn expressions(&self) -> &Vec<()> {
        // TODO Vec<Expression>
        &self.0
    }
}

impl Display for ExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO write!(f, "({})", self.0.iter().format(", "))
        Ok(())
    }
}
