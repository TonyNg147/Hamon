//! Custom error handling for pipeline processing.
#[derive(Debug)]
pub enum PipelineError {
    Malformed,
    InvalidTransformation(String),
    Misc,
}

/// Exposed Result specifically for hamon utilities.
pub type Result<T> = std::result::Result<T, PipelineError>;

/// Consitute an error that conforms with current system error handling.
/// ```rust,no_run
/// // Create an compatible error from the standard one
/// let hamon_error = Err(hamon!("Cannot convert the input {}", 2));
/// ```
#[macro_export]
macro_rules! hamon {
    // Matches a string literal or expression and wraps it
    ($msg:expr) => {
        PipelineError::InvalidTransformation($msg.into())
    };

    // Optional: Matches format strings like hamon!("Error: {}", code)
    ($fmt:expr, $($arg:tt)*) => {
        PipelineError::InvalidTransformation(format!($fmt, $($arg)*))
    };
}

/// Trait requires to extend the standard Result to the hamon result
///
/// ```rust
/// use hamon::errors::Context;
/// let io_res = Ok(5).step_err("This will be converted to Hamon error type system");
/// ```
pub trait Context<T> {
    fn step_err(self, msg: String) -> Result<T>;
}

impl<T, E> Context<T> for std::result::Result<T, E> {
    // TODO: provide the history for the error (hierarchy)
    //       Current approach doesn't pay regard to the previous
    //       instead instigating a newfangled one.
    fn step_err(self, msg: String) -> Result<T> {
        self.map_err(|_| hamon!(msg))
    }
}
