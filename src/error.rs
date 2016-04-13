use std::io;
use config;

#[derive(Debug)]
pub enum Error {
	IoError(io::Error),
	Config(config::Error),	
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IoError(error)
	}
}

impl From<config::Error> for Error {
	fn from(error: config::Error) -> Self {
		Error::Config(error)
	}
}
