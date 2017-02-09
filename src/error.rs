use std::{error, fmt};
use serde::de;

//
// TODO make the Error generic between ser and de
//
#[derive(Debug)]
pub enum Error {
	None,
	UnexpectedEOF,
	SyntaxError,
	InvalidSize,
}

impl de::Error for Error {
	fn custom<T: fmt::Display>(_msg: T) -> Self {
		Error::SyntaxError
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		"Error"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error")
	}
}