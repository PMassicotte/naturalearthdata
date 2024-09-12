/// The different scales of resolution for geographical data.
pub enum Scale {
    /// Small resolution at a scale of 1:110m.
    Small,
    /// Medium resolution at a scale of 1:50m.
    Medium,
    /// Large resolution at a scale of 1:10m.
    Large,
}

impl Scale {
    /// Returns the scale suffix as a string.
    pub fn scale_label(&self) -> &str {
        match self {
            Scale::Small => "10m",
            Scale::Medium => "50m",
            Scale::Large => "110m",
        }
    }
}
