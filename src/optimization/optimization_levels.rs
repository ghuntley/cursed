//! Optimization level configurations

pub struct LevelConfig {
    pub passes: Vec<String>,
    pub aggressive: bool,
}

impl LevelConfig {
    pub fn new() -> Self {
        LevelConfig {
            passes: vec![],
            aggressive: false,
        }
    }
}
