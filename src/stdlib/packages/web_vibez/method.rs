/// fr fr HTTP method definitions for web_vibez - all the methods you need bestie
use std::fmt;
use std::str::FromStr;

/// fr fr HTTP methods enum - covering all the standard methods no cap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// fr fr GET method for fetching resources - basic vibes
    Get,
    /// fr fr POST method for creating/submitting data - send it
    Post,
    /// fr fr PUT method for updating resources - replace it all
    Put,
    /// fr fr DELETE method for removing resources - yeet it
    Delete,
    /// fr fr PATCH method for partial updates - just tweak it
    Patch,
    /// fr fr HEAD method for headers only - metadata check
    Head,
    /// fr fr OPTIONS method for CORS and capabilities - what can we do
    Options,
    /// fr fr TRACE method for debugging - rarely used but available
    Trace,
    /// fr fr CONNECT method for tunneling - advanced networking
    Connect,
}

impl HttpMethod {
    /// fr fr Get all standard HTTP methods - complete set
    pub fn all() -> Vec<HttpMethod> {
        vec![
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Put,
            HttpMethod::Delete,
            HttpMethod::Patch,
            HttpMethod::Head,
            HttpMethod::Options,
            HttpMethod::Trace,
            HttpMethod::Connect,
        ]
    }

    /// fr fr Check if method is safe (read-only) - no side effects
    pub fn is_safe(&self) -> bool {
        matches!(self, HttpMethod::Get | HttpMethod::Head | HttpMethod::Options | HttpMethod::Trace)
    }

    /// fr fr Check if method is idempotent - same result every time
    pub fn is_idempotent(&self) -> bool {
        matches!(
            self,
            HttpMethod::Get
                | HttpMethod::Head
                | HttpMethod::Put
                | HttpMethod::Delete
                | HttpMethod::Options
                | HttpMethod::Trace
        )
    }

    /// fr fr Check if method can have a request body - data sending vibes
    pub fn can_have_body(&self) -> bool {
        matches!(
            self,
            HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch | HttpMethod::Delete
        )
    }

    /// fr fr Get method as uppercase string - standard format
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Connect => "CONNECT",
        }
    }

    /// fr fr Get method description - helpful for debugging
    pub fn description(&self) -> &'static str {
        match self {
            HttpMethod::Get => "Retrieve data from the server",
            HttpMethod::Post => "Send data to create a new resource",
            HttpMethod::Put => "Update or create a resource",
            HttpMethod::Delete => "Remove a resource from the server",
            HttpMethod::Patch => "Partially update a resource",
            HttpMethod::Head => "Get headers without response body",
            HttpMethod::Options => "Get allowed methods and CORS info",
            HttpMethod::Trace => "Echo the request for debugging",
            HttpMethod::Connect => "Establish a tunnel connection",
        }
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
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            "HEAD" => Ok(HttpMethod::Head),
            "OPTIONS" => Ok(HttpMethod::Options),
            "TRACE" => Ok(HttpMethod::Trace),
            "CONNECT" => Ok(HttpMethod::Connect),
            _ => Err(InvalidMethodError {
                method: s.to_string(),
            }),
        }
    }
}

/// fr fr Error type for invalid HTTP methods - when parsing fails
#[derive(Debug, Clone)]
pub struct InvalidMethodError {
    pub method: String,
}

impl fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP method: '{}'", self.method)
    }
}

impl std::error::Error for InvalidMethodError {}

/// fr fr Helper macro for method matching in handlers - clean syntax
#[macro_export]
macro_rules! match_method {
    ($method:expr, {
        GET => $get_handler:expr,
        POST => $post_handler:expr,
        $(PUT => $put_handler:expr,)?
        $(DELETE => $delete_handler:expr,)?
        $(PATCH => $patch_handler:expr,)?
        $(_ => $default_handler:expr)?
    }) => {
        match $method {
            HttpMethod::Get => $get_handler,
            HttpMethod::Post => $post_handler,
            $(HttpMethod::Put => $put_handler,)?
            $(HttpMethod::Delete => $delete_handler,)?
            $(HttpMethod::Patch => $patch_handler,)?
            $(_ => $default_handler)?
        }
    };
}

/// fr fr Method set for allowed methods on endpoints - CORS support
#[derive(Debug, Clone)]
pub struct MethodSet {
    methods: Vec<HttpMethod>,
}

impl MethodSet {
    /// fr fr Create new method set - start fresh
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
        }
    }

    /// fr fr Create method set with specific methods - quick setup
    pub fn from_methods(methods: Vec<HttpMethod>) -> Self {
        let mut set = Self::new();
        for method in methods {
            set.add(method);
        }
        set
    }

    /// fr fr Add method to set - expand capabilities
    pub fn add(&mut self, method: HttpMethod) {
        if !self.methods.contains(&method) {
            self.methods.push(method);
        }
    }

    /// fr fr Remove method from set - restrict access
    pub fn remove(&mut self, method: HttpMethod) {
        self.methods.retain(|&m| m != method);
    }

    /// fr fr Check if method is allowed - quick validation
    pub fn contains(&self, method: HttpMethod) -> bool {
        self.methods.contains(&method)
    }

    /// fr fr Get all methods in set - for headers
    pub fn methods(&self) -> &[HttpMethod] {
        &self.methods
    }

    /// fr fr Convert to Allow header value - CORS compliance
    pub fn to_allow_header(&self) -> String {
        self.methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// fr fr Check if set is empty - validation helper
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_properties() {
        assert!(HttpMethod::Get.is_safe());
        assert!(!HttpMethod::Post.is_safe());
        
        assert!(HttpMethod::Put.is_idempotent());
        assert!(!HttpMethod::Post.is_idempotent());
        
        assert!(HttpMethod::Post.can_have_body());
        assert!(!HttpMethod::Get.can_have_body());
    }

    #[test]
    fn test_method_from_str() {
        assert_eq!("GET".parse::<HttpMethod>().unwrap(), HttpMethod::Get);
        assert_eq!("post".parse::<HttpMethod>().unwrap(), HttpMethod::Post);
        assert!("INVALID".parse::<HttpMethod>().is_err());
    }

    #[test]
    fn test_method_set() {
        let mut set = MethodSet::new();
        assert!(set.is_empty());
        
        set.add(HttpMethod::Get);
        set.add(HttpMethod::Post);
        assert_eq!(set.len(), 2);
        assert!(set.contains(HttpMethod::Get));
        assert!(!set.contains(HttpMethod::Delete));
        
        assert_eq!(set.to_allow_header(), "GET, POST");
    }

    #[test]
    fn test_method_descriptions() {
        assert_eq!(HttpMethod::Get.description(), "Retrieve data from the server");
        assert_eq!(HttpMethod::Post.description(), "Send data to create a new resource");
    }
}
