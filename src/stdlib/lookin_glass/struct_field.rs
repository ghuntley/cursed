/// StructField represents a field in a squad (struct) for CURSED reflection
// use crate::stdlib::lookin_glass::{Type, StructTag};
use std::fmt;

/// Represents a field in a squad (struct)
#[derive(Debug, Clone)]
pub struct StructField {
    /// Name of the field
    /// Package path where the field is defined (empty for exported fields)
    /// Type of the field
    /// Struct tag associated with the field
    /// Byte offset of field within struct
    /// Index path for nested anonymous fields
    /// Whether this is an anonymous (embedded) field
impl StructField {
    /// Create a new StructField
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Create a simple field with just name and type
    pub fn simple(name: String, field_type: Type) -> Self {
        Self {
        }
    }

    /// Get the field name
    pub fn name(&self) -> &str {
        &self.name
    /// Get the package path
    pub fn pkg_path(&self) -> &str {
        &self.pkg_path
    /// Get the field type
    pub fn field_type(&self) -> &Type {
        &self.field_type
    /// Get the struct tag
    pub fn tag(&self) -> &StructTag {
        &self.tag
    /// Get the byte offset within the struct
    pub fn offset(&self) -> usize {
        self.offset
    /// Get the index path for accessing this field
    pub fn index(&self) -> &[usize] {
        &self.index
    /// Check if this is an anonymous (embedded) field
    pub fn is_anonymous(&self) -> bool {
        self.anonymous
    /// Check if this field is exported (accessible from other packages)
    pub fn is_exported(&self) -> bool {
        self.pkg_path.is_empty() && 
        self.name.chars().next().map_or(false, |c| c.is_uppercase())
    /// Check if this field can be set (is addressable and exported)
    pub fn can_set(&self) -> bool {
        self.is_exported()
    /// Get the depth of this field (length of index path)
    pub fn depth(&self) -> usize {
        self.index.len()
    /// Check if this field is at the top level (not embedded)
    pub fn is_top_level(&self) -> bool {
        self.index.len() == 1
    /// Get a tag value by key
    pub fn get_tag(&self, key: &str) -> String {
        self.tag.get(key)
    /// Check if the field has a specific tag
    pub fn has_tag(&self, key: &str) -> bool {
        self.tag.has_key(key)
    /// Get the JSON tag name (commonly used convention)
    pub fn json_name(&self) -> Option<String> {
        let json_tag = self.tag.get("json");
        if json_tag.is_empty() {
            return None;
        // Parse JSON tag (format: "name,option1,option2")
        let parts: Vec<&str> = json_tag.split(',').collect();
        if let Some(first) = parts.first() {
            if first.trim() == "-" {
                None // Explicitly excluded from JSON
            } else if first.trim().is_empty() {
                Some(self.name.clone()) // Use field name
            } else {
                Some(first.trim().to_string())
            }
        } else {
            Some(self.name.clone())
        }
    }

    /// Check if this field should be omitted when empty (omitempty tag)
    pub fn omit_empty(&self) -> bool {
        let json_tag = self.tag.get("json");
        json_tag.contains("omitempty")
    /// Check if this field is excluded from JSON serialization
    pub fn json_ignored(&self) -> bool {
        let json_tag = self.tag.get("json");
        json_tag.trim() == "-"
    /// Get database column name (db tag)
    pub fn db_column(&self) -> Option<String> {
        let db_tag = self.tag.get("db");
        if db_tag.is_empty() || db_tag.trim() == "-" {
            None
        } else {
            Some(db_tag)
        }
    }

    /// Get validation rules (validate tag)
    pub fn validation_rules(&self) -> Vec<String> {
        let validate_tag = self.tag.get("validate");
        if validate_tag.is_empty() {
            Vec::new()
        } else {
            validate_tag.split(',').map(|s| s.trim().to_string()).collect()
        }
    }

    /// Create a field builder for fluent construction
    pub fn builder(name: String, field_type: Type) -> StructFieldBuilder {
        StructFieldBuilder::new(name, field_type)
    }
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.field_type)?;
        if !self.tag.is_empty() {
            write!(f, " `{}`", self.tag)?;
        }
        Ok(())
    }
}

/// Builder for creating StructField instances
pub struct StructFieldBuilder {
impl StructFieldBuilder {
    /// Create a new builder
    pub fn new(name: String, field_type: Type) -> Self {
        Self {
        }
    }

    /// Set the package path
    pub fn pkg_path(mut self, pkg_path: String) -> Self {
        self.pkg_path = pkg_path;
        self
    /// Set the struct tag
    pub fn tag(mut self, tag: StructTag) -> Self {
        self.tag = tag;
        self
    /// Set the struct tag from string
    pub fn tag_string(mut self, tag: String) -> Self {
        self.tag = StructTag::new(tag);
        self
    /// Set the field offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    /// Set the index path
    pub fn index(mut self, index: Vec<usize>) -> Self {
        self.index = index;
        self
    /// Set whether this is an anonymous field
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = anonymous;
        self
    /// Build the StructField
    pub fn build(self) -> StructField {
        StructField {
        }
    }
