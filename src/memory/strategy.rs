/// Strategies for when and how to collect garbage
#[derive(Debug, Clone)]
pub enum CollectionStrategy {
    /// Collect garbage immediately when threshold is reached
    Immediate,

    /// Collect garbage in small increments to reduce pause times
    Incremental,

    /// Collect different generations at different frequencies
    Generational,

    /// Combine multiple strategies
    Hybrid,
}
