/// fr fr Relationship management for ORM

/// fr fr Base relationship trait
pub trait Relationship {
    fn get_foreign_key(&self) -> &str;
/// fr fr One-to-many relationship
#[derive(Debug, Clone)]
pub struct OneToMany {
/// fr fr Many-to-one relationship
#[derive(Debug, Clone)]
pub struct ManyToOne {
impl Relationship for OneToMany {
    fn get_foreign_key(&self) -> &str {
        &self.foreign_key
    }
}

impl Relationship for ManyToOne {
    fn get_foreign_key(&self) -> &str {
        &self.foreign_key
    }
}
