#[derive(Debug)]
pub enum PipelineError {
    Malformed,
    InvalidTransformation(String),
    Misc,
}

pub type Result<T> = std::result::Result<T, PipelineError>;

#[macro_export]
macro_rules! hamon {
    // Matches a string literal or expression and wraps it
    ($msg:expr) => {
        $crate::PipelineError::InvalidTransformation($msg.to_string())
    };

    // Optional: Matches format strings like hamon!("Error: {}", code)
    ($fmt:expr, $($arg:tt)*) => {
        $crate::PipelineError::InvalidTransformation(format!($fmt, $($arg)*))
    };
}
