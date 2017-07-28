use ruwm::xcb::base;
use ruwm::colored::Colorize;
use std::error;
use std::fmt;

/* A simple error enum for handling ruwm errors
 * Panics upon error, with a red error message
 * Need to eventually implement Err trait for
 * err.rs 
 */

// there is no error for not acquiring a screen in the XCB Rust Library bindings
// So I made one

#[derive(Debug)]
pub struct CouldNotAcquireScreen;

pub type CouldNotAcquireScreenError = CouldNotAcquireScreen;

impl fmt::Display for CouldNotAcquireScreenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Could Not Acquire Screen Error")
	}
}
impl error::Error for CouldNotAcquireScreenError {
	fn description(&self) -> &str {
		"The Screen Could Not Be Found, is Taken by Another WM, or X was not able to lock it down."
	}
}


#[derive(Debug)]
pub enum RuwmError {
	// Could not connect tothe X server
	CouldNotConnect(base::ConnError),
	// Could not acquire a scren from X server
	CouldNotAcquireScreen(CouldNotAcquireScreenError),
	// An atom used by the WM wasn't accepted by the X server
	CouldNotRegisterAtom(base::GenericError),
	// Another WM was running and ruwm couldn't start
	// OtherWmRunning,
	// Input/Output with X Server could not work
	// IOError,
	// Could not configure Bindings
	// TODO eventually, instead of totally killing the WM
	// Do what i3 does, keep default/last good bindings
	// and display a nice terminal window in vim/emacs/nano
	// with errors
	// CouldNotConfigureBindings(String),
}

impl fmt::Display for RuwmError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			RuwmError::CouldNotConnect(ref err) => write!(f, "Could Not Connect Error {}", err),
			RuwmError::CouldNotAcquireScreen(ref err) => write!(f, "Could Not Acquire a screen from the X Server {}", err),
			RuwmError::CouldNotRegisterAtom(ref err) => write!(f, "Could Not Register Atom {}", err),
		}
	}
}

impl error::Error for RuwmError {
	fn description(&self) -> &str {
		match *self {
			RuwmError::CouldNotConnect(ref err) => err.description(),
			RuwmError::CouldNotAcquireScreen(ref err ) => err.description(), // Does not have an error type
			RuwmError::CouldNotRegisterAtom(ref err) => err.description(), // this might be a really bad error message
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match *self {
			RuwmError::CouldNotConnect(ref err) => Some(err),
			RuwmError::CouldNotAcquireScreen(ref err) => Some(err),
			RuwmError::CouldNotRegisterAtom(ref err) => Some(err),
		}
	}
}


// From Conversions
impl From<base::ConnError> for RuwmError {
	fn from(err: base::ConnError ) -> RuwmError {
		RuwmError::CouldNotConnect(err)
	}
}

impl From<CouldNotAcquireScreenError> for RuwmError {
	fn from(err: CouldNotAcquireScreenError ) -> RuwmError {
		RuwmError::CouldNotAcquireScreen(err)
	}
}

impl From<base::GenericError> for RuwmError {
	fn from(err: base::GenericError ) -> RuwmError {
		RuwmError::CouldNotRegisterAtom(err)
	}
}