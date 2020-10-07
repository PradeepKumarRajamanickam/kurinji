use serde::{Deserialize, Serialize};

/// Axis of mouse movement
#[derive(Serialize, Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    XPositive,
    XNegative,

    YPositive,
    YNegative,
}

/// Axis of gamepad analog
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GamepadAxis {
    LeftStickXPositive,
    LeftStickXNegative,

    LeftStickYPositive,
    LeftStickYNegative,

    LeftZPositive,
    LeftZNegative,

    RightStickXPositive,
    RightStickXNegative,

    RightStickYPositive,
    RightStickYNegative,

    RightZPositive,
    RightZNegative,

    DPadXPositive,
    DPadXNegative,

    DPadYPositive,
    DPadYNegative,
}

impl Default for GamepadAxis {
    fn default() -> Self {
        GamepadAxis::LeftStickXPositive
    }
}