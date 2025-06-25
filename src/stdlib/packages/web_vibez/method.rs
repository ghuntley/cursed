use crate::error::CursedError;
/// fr fr HTTP method definitions for web_vibez - all the methods you need bestie
use std::fmt;
use std::str::FromStr;

/// fr fr HTTP methods enum - covering all the standard methods no cap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// fr fr GET method for fetching resources - basic vibes
    /// fr fr POST method for creating/submitting data - send it
    /// fr fr PUT method for updating resources - replace it all
    /// fr fr DELETE method for removing resources - yeet it
    /// fr fr PATCH method for partial updates - just tweak it
    /// fr fr HEAD method for headers only - metadata check
    /// fr fr OPTIONS method for CORS and capabilities - what can we do
    /// fr fr TRACE method for debugging - rarely used but available
    /// fr fr CONNECT method for tunneling - advanced networking
impl HttpMethod {
    /// fr fr Get all standard HTTP methods - complete set
    pub fn all() -> Vec<HttpMethod> {
        vec![
        ]
    /// fr fr Check if method is safe (read-only) - no side effects
    pub fn is_safe(&self) -> bool {
        matches!(self, HttpMethod::Get | HttpMethod::Head | HttpMethod::Options | HttpMethod::Trace)
    /// fr fr Check if method is idempotent - same result every time
    pub fn is_idempotent(&self) -> bool {
        matches!(
            HttpMethod::Get
                | HttpMethod::Head
                | HttpMethod::Put
                | HttpMethod::Delete
                | HttpMethod::Options
                | HttpMethod::Trace
        )
    /// fr fr Check if method can have a request body - data sending vibes
    pub fn can_have_body(&self) -> bool {
        matches!(
            HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch | HttpMethod::Delete
        )
    /// fr fr Get method as uppercase string - standard format
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }

    /// fr fr Get method description - helpful for debugging
    pub fn description(&self) -> &'static str {
        match self {
        }
    }
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for HttpMethod {
    type Err = InvalidMethodError;

    /// fr fr Parse HTTP method from string - case insensitive
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            _ => Err(InvalidMethodError {
        }
    }
/// fr fr CursedError type for invalid HTTP methods - when parsing fails
#[derive(Debug, Clone)]
pub struct InvalidMethodError {
// impl fmt::Display for InvalidMethodError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Invalid HTTP method: '{}'", self.method)
//     }
// }

// impl std::error::CursedError for InvalidMethodError {}
// 
/// fr fr Helper macro for method matching in handlers - clean syntax
#[macro_export]
macro_rules! match_method {
    ($method:expr, {
        $(PUT => $put_handler:expr,)?
        $(DELETE => $delete_handler:expr,)?
        $(PATCH => $patch_handler:expr,)?
        $(_ => $default_handler:expr)?
    }) => {
        match $method {
            $(HttpMethod::Put => $put_handler,)?
            $(HttpMethod::Delete => $delete_handler,)?
            $(HttpMethod::Patch => $patch_handler,)?
            $(_ => $default_handler)?
        }
/// fr fr Method set for allowed methods on endpoints - CORS support
#[derive(Debug, Clone)]
pub struct MethodSet {
impl MethodSet {
    /// fr fr Create new method set - start fresh
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Create method set with specific methods - quick setup
    pub fn from_methods(methods: Vec<HttpMethod>) -> Self {
        let mut set = Self::new();
        for method in methods {
            set.add(method);
        }
        set
    /// fr fr Add method to set - expand capabilities
    pub fn add(&mut self, method: HttpMethod) {
        if !self.methods.contains(&method) {
            self.methods.push(method);
        }
    }

    /// fr fr Remove method from set - restrict access
    pub fn remove(&mut self, method: HttpMethod) {
        self.methods.retain(|&m| m != method);
    /// fr fr Check if method is allowed - quick validation
    pub fn contains(&self, method: HttpMethod) -> bool {
        self.methods.contains(&method)
    /// fr fr Get all methods in set - for headers
    pub fn methods(&self) -> &[HttpMethod] {
        &self.methods
    /// fr fr Convert to Allow header value - CORS compliance
    pub fn to_allow_header(&self) -> String {
        self.methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    /// fr fr Check if set is empty - validation helper
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    /// fr fr Get method count - stats
    pub fn len(&self) -> usize {
        self.methods.len()
    }
}

impl Default for MethodSet {
    /// fr fr Default allows GET and POST - common setup
    fn default() -> Self {
        Self::from_methods(Vec::from([HttpMethod::Get, HttpMethod::Post]))
    }
}

impl fmt::Display for MethodSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_allow_header())
    }
}

