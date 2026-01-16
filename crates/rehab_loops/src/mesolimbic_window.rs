pub struct MesolimbicRewardWindow {
    /// Anticipated delay from cortical prediction signal
    /// to peak NAc BOLD-equivalent response.
    pub nac_delay_s: f32,
    /// Maximum window for safely stacking salient events
    /// without approximating drug-like rapid reinforcement.
    pub stack_window_s: f32,
}

impl MesolimbicRewardWindow {
    pub fn rehab_default() -> Self {
        // Values chosen from hemodynamic timing literature:
        // ~4–6 s NAc peak; stacking window widened to
        // discourage rapid, slot-machine-like reward pacing.
        MesolimbicRewardWindow {
            nac_delay_s: 5.0,
            stack_window_s: 8.0,
        }
    }

    /// Returns true if the next salient audio event
    /// is spaced far enough from the previous one
    /// to avoid fast reinforcement timing.
    pub fn is_rehab_safe_spacing(&self, last_event_t: f32, next_event_t: f32) -> bool {
        let dt = next_event_t - last_event_t;
        dt >= self.stack_window_s
    }
}
