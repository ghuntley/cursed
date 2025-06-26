//! REPL Session Management

use crate::error::CursedError;
use std::collections::VecDeque;

pub struct SessionManager {
    history: VecDeque<String>,
    max_history: usize,
    session_id: String,
}

impl SessionManager {
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            history: VecDeque::new(),
            max_history: 1000,
            session_id: format!("session_{}", std::process::id()),
        })
    }

    pub fn add_to_history(&mut self, entry: String) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(entry);
    }

    pub fn get_history(&self) -> &VecDeque<String> {
        &self.history
    }

    pub fn get_last_entries(&self, count: usize) -> Vec<&String> {
        self.history.iter().rev().take(count).collect()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn save_session(&self, _path: &str) -> Result<(), CursedError> {
        // TODO: Implement session persistence
        Ok(())
    }

    pub fn load_session(&mut self, _path: &str) -> Result<(), CursedError> {
        // TODO: Implement session loading
        Ok(())
    }
}
