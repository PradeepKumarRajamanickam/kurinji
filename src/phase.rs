use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
/// Event phase that action is mapped to.
pub enum Phase
{
    OnBegin, // In the beginning of the event
    OnProgress, // During the event
    OnEnded, // At the end of the event
}
impl Default for Phase {
    fn default() -> Self { Phase::OnProgress }
}