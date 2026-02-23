#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MotionPreset {
    None,
    Subtle,
    Cinematic,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MotionTokens {
    pub duration_fast_ms: u32,
    pub duration_base_ms: u32,
    pub duration_slow_ms: u32,
    pub easing_enter: String,
    pub easing_emphasis: String,
    pub stagger_ms: u32,
    pub travel_px: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MotionSpec {
    pub hero: MotionPreset,
    pub sections: MotionPreset,
    pub cards: MotionPreset,
}

impl Default for MotionSpec {
    fn default() -> Self {
        Self {
            hero: MotionPreset::Subtle,
            sections: MotionPreset::Subtle,
            cards: MotionPreset::Subtle,
        }
    }
}
