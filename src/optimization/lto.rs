//! Link-time optimization support

#[derive(Debug, Clone)]
pub struct LtoConfig {
    pub enabled: bool,
    pub level: LtoLevel,
}

#[derive(Debug, Clone)]
pub enum LtoLevel {
    None,
    Thin,
    Fat,
}

pub struct LtoCompilationUnit;
pub struct LtoStatistics;
