use ruwm::xcb::base;
use ruwm::colored::Colorize;

// A simple error enum for handling ruwm errors

#[derive(Debug)]
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
	// Could not configure Bindings
	// TODO eventually, instead of totally killing the WM
	// Do what i3 does, keep default/last good bindings
	// and display a nice terminal window in vim/emacs/nano
	// with errors
	CouldNotConfigureBindings(String),
}


impl RuwmError {
	/// Print an error message and exit
	// TODO create a logger fn (Maybe better to make a macro?)
	// Logger.error
	// Logger.warn
	// Logger.fatal
	// Logger.etc...etc...etc...
	// Different logs have different colors and some don't panic!
	// Logger doesn't have to only be used in errors, so worth it to make it a seperate struct
	pub fn handle(self) -> ! {
		match self {
			RuwmError::CouldNotConnect(e) => {
				return match e {
					base::ConnError::Connection => {
						panic!("{}", "Could not connect to X: socket, pipe or other stream error.".red())
					},
					base::ConnError::ClosedExtNotSupported => {
						panic!("{}", "Could not connect: X connection shutdown because extension not supported".red())
					},
					base::ConnError::ClosedMemInsufficient => {
						panic!("{}", "(Insufficient Memory) malloc(), calloc(), and realloc() error upon failure, for `eg: ENOMEM`".red())
					},
					base::ConnError::ClosedReqLenExceed => {
						panic!("{}", "Connection closed, exceeding request length that server accepts".red())
					},
					base::ConnError::ClosedParseErr => {
						panic!("{}", "Connection closed, error during parsing display string".red())
					},
					base::ConnError::ClosedInvalidScreen => {
						panic!("{}", "Connection closed because server does not have a screen matching the display".red())
					},
					base::ConnError::ClosedFdPassingFailed => {
						panic!("{}", "Connection closed because File Descriptor passing operation failed".red())
					}
				};
			},
			RuwmError::CouldNotAcquireScreen => panic!("{}", "Could not acquire screen".red()),
			RuwmError::CouldNotRegisterAtom(e) => panic!("{} {}", "Could not register atom".red(), e.bold().red()),
			RuwmError::ConnectionInterrupted => panic!("{}", "Another WM is running, ruwm couldn't connect :-(".red()),
			RuwmError::IOError => panic!("{}", "IO Error!".red()),
			RuwmError::CouldNotConfigureBindings(e) => panic!("{} {}", "Could not configure bindings".red(), e.red()),
			_ => panic!("{}", "Something unexpected happened! Exiting...".red()),
		};
	}
}
