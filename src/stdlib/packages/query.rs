/// Database query types
#[derive(Debug, Clone)]
pub struct Query {
    pub sql: String,
    pub parameters: Vec<String>,
}


