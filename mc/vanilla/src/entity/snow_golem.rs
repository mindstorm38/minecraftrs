#[derive(Debug)]
pub struct SnowGolemEntity {
    /// Whether or not the Snow Golem has a pumpkin on its head. True by default.
    pumpkin: bool
}

impl Default for SnowGolemEntity {
    fn default() -> Self {
        Self {
            pumpkin: true
        }
    }
}
