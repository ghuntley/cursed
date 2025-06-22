/// Database parameter types
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterDirection {
    In,
    Out,
    InOut,
    Return,
}
