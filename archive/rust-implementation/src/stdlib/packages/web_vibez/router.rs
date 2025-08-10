use std::collections::HashMap;

/// Router for handling HTTP routes
pub struct Router {
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Route entry
pub struct RouteEntry {
    pub path: String,
    pub method: String,
}

/// Path parameters
pub struct PathParams {
    pub params: HashMap<String, String>,
}
