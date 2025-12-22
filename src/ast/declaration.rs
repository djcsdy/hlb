use crate::ast::{ExportDeclaration, FunctionDeclaration, ImportDeclaration};
use derive_more::{Display, From, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, From, IsVariant, Unwrap, TryUnwrap,
)]
pub enum Declaration {
    Import(ImportDeclaration),
    Export(ExportDeclaration),
    Function(FunctionDeclaration),
}
