use serde::{Deserialize, Serialize};


/// Axis of mouse movement
#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    XPositive,
    XNegative,

    YPositive,
    YNegative,
}

/// Gamepad Analog move direction
#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum AnalogDirection {
    Positve,
    Negative,
}
