mod parser;
mod expressions;
mod statements;
mod types;
mod precedence;

#[cfg(test)]
mod tests;

pub use parser::Parser;