//! Memory visitor implementations

use crate::memory::Tag;

/// Visit memory objects during tracing
pub trait Visitor {
    /// Visit a raw pointer
    fn visit_ptr(&mut self, address: usize, tag: Tag);

    /// Visit a raw pointer with context
    fn visit_ptr_with_context(&mut self, address: usize, tag: Tag, context: &str);
}

/// Default visitor implementation
pub struct DefaultVisitor;

impl Visitor for DefaultVisitor {
    fn visit_ptr(&mut self, _address: usize, _tag: Tag) {
        // No-op implementation
    }

    fn visit_ptr_with_context(&mut self, _address: usize, _tag: Tag, _context: &str) {
        // No-op implementation
    }
}
