use derive_more::{Display, IsVariant, TryUnwrap, Unwrap};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Display, IsVariant, Unwrap, TryUnwrap,
)]
pub enum Modifier {
    #[display("variadic")]
    Variadic,
}
