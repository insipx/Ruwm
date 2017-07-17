use std::process::exit;
use xcb::base;

// A simple error enum for handling ruwm errors
pub enum RuwmError {
	// Could not connect tothe X server
	CouldNotConnect(base::ConnError),
	// Could not acquire a scren from X server
	CouldNotAcquireScreen,
	// An atom used by the WM wasn't accepted by the X server
	CouldNotRegisterAtom(String),
	// Connection to the X server was interrupted
	ConnectionInterrupted,
	// Another WM was running and ruwm couldn't start
	OtherWmRunning,
	// Input/Output with X Server could not work
	IOError,
}


impl RuwmError {
	/// Print an error message and exit
	fn handle(self) -> ! {
		match self {
			RuwmError::CouldNotConnect(e) => error!("Could not connect {:?}", e),
			RuwmError::CouldNotAcquireScreen => error!("Could not acquire screen"),
			RuwmError::CouldNotRegisterAtom(e) => error!("Could not register atom {}", e),
			RuwmError::ConnectionInterrupted => error!("Another WM is running, ruwm couldn't connect :-("),
			RuwmError::IOError => error!("IO Error!")
			_ => error!("Something unexpected happened! Exiting..."),
		};
		exit(1)
	}

}
