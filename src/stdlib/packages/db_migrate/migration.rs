/// fr fr Migration definitions and scripts

use std::time::SystemTime;

/// fr fr Database migration
#[derive(Debug, Clone)]
pub struct Migration {
/// fr fr Migration script
#[derive(Debug, Clone)]
pub struct MigrationScript {
impl Migration {
    pub fn new(id: &str, name: &str, version: u64) -> Self {
        Self {
            up_script: MigrationScript {
            down_script: MigrationScript {
        }
    }
}
