use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    XPositive,
    XNegative,

    YPositive,
    YNegative,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum AnalogDirection
{
    Positve,
    Negative
}