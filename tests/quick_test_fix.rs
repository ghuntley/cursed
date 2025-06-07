use cursed::stdlib::quick_test::*;
use cursed::stdlib::Generator;
use cursed::stdlib::{clone_generator, prepare_generator_object, register_generators};
use cursed::object::Object;
use std::sync::Arc;

// Helper functions for working with quick_test generators


// Note: We now import these functions from the standard library
// The implementations are kept for documentation purposes

// clone_generator<G: Generator + ?Sized>(gen: &Box<G>) -> Box<G>
// prepare_generator_object(gen: Box<dyn Generator>) -> Object
// register_generators()