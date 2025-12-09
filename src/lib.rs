pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod args;
pub mod solution;
pub mod year2025;
