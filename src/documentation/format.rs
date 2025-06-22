/// Documentation format definitions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocFormat {
    Markdown,
    Html,
    Json,
    Xml,
    Text,
}

impl DocFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            DocFormat::Markdown => "md",
            DocFormat::Html => "html",
            DocFormat::Json => "json",
            DocFormat::Xml => "xml",
            DocFormat::Text => "txt",
        }
    }
}
