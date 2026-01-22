#[derive(Clone, Copy, Debug)]
pub struct FearPainEpoch {
    pub fear_level: f32,        // 0..1, from FearWeightProfile
    pub pain_level: f32,        // 0..1, tolerable only
    pub stability_score: f32,   // 0..1, from NeuroswarmGuard
    pub autonomic_risk: f32,    // 0..1, from LF/HF + AII
    pub inactive_flag: bool,    // true if N2/N3 inactive consciousness
}

impl FearPainEpoch {
    pub fn learning_window(&self,
                           f_max: f32,
                           p_max: f32) -> f32 {
        if !self.inactive_flag {
            return 0.0;
        }
        let f = clamp01(self.fear_level).min(clamp01(f_max));
        let p = clamp01(self.pain_level).min(clamp01(p_max));
        let s = clamp01(self.stability_score);
        let a = clamp01(self.autonomic_risk);
        f * p * s * (1.0 - a)
    }
}

fn clamp01(x: f32) -> f32 {
    if !x.is_finite() { return 0.0; }
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}
