use crate::Kurinji;
use serde::{Deserialize, Serialize};
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Copy
)]
/// Event phase that action is mapped to.
pub enum EventPhase {
    OnBegin,    // In the beginning of the event
    OnProgress, // During the event
    OnEnded,    // At the end of the event
}
impl Default for EventPhase {
    fn default() -> Self {
        EventPhase::OnProgress
    }
}
impl Kurinji {
    // publics
    /// Returns in which event phase this action active will be true
    pub fn get_event_phase(&self, action: &str) -> &EventPhase {
        if let Some(v) = self.action_phase.get(action) {
            return v;
        }
        &EventPhase::OnProgress
    }

    /// Set on which event phase should action will be true.
    /// By default will be Phase::OnProgress
    pub fn set_event_phase(
        &mut self,
        action: &str,
        phase: EventPhase,
    ) -> &mut Kurinji {
        self.action_phase.insert(action.to_string(), phase);
        self
    }

    // crates
    pub(crate) fn did_action_just_began(&self, action: &str) -> bool {
        self.get_prev_strength(action) == 0.0
            && self.get_action_strength(action) > 0.0
    }

    /// Is this action happening.
    /// Note* this does not consider action event phase
    /// use is_action_active() in that case
    pub(crate) fn is_action_in_progress(&self, action: &str) -> bool {
        self.get_action_strength(action) > 0.0
    }

    pub(crate) fn did_action_just_end(&self, action: &str) -> bool {
        self.get_prev_strength(action) > 0.0
            && self.get_action_strength(action) == 0.0
    }
}
