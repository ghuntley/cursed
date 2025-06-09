/// Memory management system
pub mod gc;

pub use gc::GarbageCollector;

pub trait Traceable {
    fn trace(&self, visitor: &mut dyn Visitor);
}

pub trait Visitor {
    fn visit(&mut self, obj: &dyn Traceable);
}

pub enum Tag {
    Object,
    Array,
    Function,
}
