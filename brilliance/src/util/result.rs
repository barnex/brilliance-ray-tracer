use std::error;
use std::fmt;
use std::result;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn error::Error>;

// Generic error. Only has an error message.
pub struct GenError {
	inner: String,
}

pub fn error<T>(inner: String) -> Result<T> {
	GenError::new(inner)
}

impl GenError {
	pub fn new<T>(inner: String) -> Result<T> {
		Result::Err(Box::new(GenError { inner }))
	}
}

impl fmt::Display for GenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
	}
}

impl fmt::Debug for GenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		write!(f, "error: {}", self.inner)
	}
}

impl error::Error for GenError {}
