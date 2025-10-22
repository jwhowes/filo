use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct RuntimeError(String);

impl RuntimeError {
	pub fn new(message: impl Into<String>) -> Self {
		Self(message.into())
	}
}

impl Display for RuntimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Runtime error: {}", self.0)
	}
}

impl Error for RuntimeError {}

pub type RuntimeResult<T> = Result<T, RuntimeError>;