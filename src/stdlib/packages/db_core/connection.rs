//! Database connection traits - MINIMAL FOR CURSED RESTORATION

use crate::error::CursedError;

pub type ConnectionConfig = String;
pub type ConnectionInfo = String;
pub type DatabaseResult<T> = Result<T, CursedError>;

/// Core database connection trait
pub trait DatabaseConnection {
    fn connect(&mut self, config: &ConnectionConfig) -> DatabaseResult<()>;
    fn disconnect(&mut self) -> DatabaseResult<()>;
    fn is_connected(&self) -> bool;
}

/// Basic connection implementation
pub struct BasicConnection {
    connected: bool,
}

impl BasicConnection {
    pub fn new() -> Self {
        Self { connected: false }
    }
}

impl DatabaseConnection for BasicConnection {
    fn connect(&mut self, _config: &ConnectionConfig) -> DatabaseResult<()> {
        self.connected = true;
        Ok(())
    }
    
    fn disconnect(&mut self) -> DatabaseResult<()> {
        self.connected = false;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
}
