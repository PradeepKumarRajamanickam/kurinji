use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    XPositive,
    XNegative,

    YPositive,
    YNegative,
}

#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum AnalogDirection {
    Positve,
    Negative,
}
