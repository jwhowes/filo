use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseError(String);

impl ParseError {
	pub fn new(message: impl Into<String>) -> Self {
		Self(message.into())
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Parse error: {}", self.0)
	}
}

impl Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;