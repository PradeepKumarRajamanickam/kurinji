use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Axis {
    XPositive,
    XNegative,

    YPositive,
    YNegative,
}