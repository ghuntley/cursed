/// fr fr Migration definitions and scripts

use std::time::SystemTime;

/// fr fr Database migration
#[derive(Debug, Clone)]
pub struct Migration {
    pub id: String,
    pub name: String,
    pub version: u64,
    pub up_script: MigrationScript,
    pub down_script: MigrationScript,
    pub created_at: SystemTime,
}

/// fr fr Migration script
#[derive(Debug, Clone)]
pub struct MigrationScript {
    pub sql: String,
    pub description: String,
}

impl Migration {
    pub fn new(id: &str, name: &str, version: u64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version,
            up_script: MigrationScript {
                sql: String::new(),
                description: String::new(),
            },
            down_script: MigrationScript {
                sql: String::new(),
                description: String::new(),
            },
            created_at: SystemTime::now(),
        }
    }
}
