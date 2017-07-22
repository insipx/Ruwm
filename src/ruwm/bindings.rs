use ruwm::err::RuwmError;

use ruwm::xcb;
use std::collections::HashMap;
use ruwm::scancode::Scancode;

enum InputType {
	Keyboard,
	Mouse,
}

/*
impl InputType {


}
*/

/* A Scancode or combination of Scancodes
* Which dictate an action
* Taken from i3. Not sure if i need this,
* Just testing out xkb
*/ 
pub struct Binding {
	/*
	 * The type of input this binding is for (InputType enum)
	 */
	input_type: InputType,
	keycode: u32,
	/* 
	 * Symbol the user specified in configfile. This needs to be 
	 * stored with the binding to be able to re-convert it into a keycode
	 * if the keyboard mapping changes (using Xmodmap for example)
	 */
	// symbol: &str,

	/* Command, like in command mode */
	// command: &str,

}

impl Binding {
	pub fn new() -> Result<Binding, RuwmError> {
		unimplemented!();	
	}
}


pub struct Bindings {
	// Scancode/Scancode Combo => Action
	// pub bindings: HashMap<Binding, Action>,
}

impl Bindings {
	/*	
	pub fn new() -> Result<Bindings, RuwmError> {
		// get bindings
		// configure bindings
		// validate bindings
		// return valid bindings
		// For now, just return an empty hashmap
		Ok (Bindings {
			bindings: HashMap::new()
		})
	}
	*/
	/*
	 * Gets bindings from configuration file,
	 * and stuffs them into a datastructure 
	 * (Dictionary?)
	 *
	 */
	pub fn get_bindings(config: &str) {
		unimplemented!();
	}

	/*
	 * Configures bindings with XCB
	 *
	 */
	pub fn configure_bindings() {
		unimplemented!();
	}

	/*
	 * Check for duplicate bindings in binding
	 * Datastructure
	 *
	 */
	pub fn check_for_duplicate_bindings() {
		unimplemented!();
	}

	/* 
	 * Validate bindings 
	 * Check for duplicates
	 * And do anything else that needs to be done
	 * in order to avoid errors
	 *
	 */
	pub fn validate_bindings() {
		unimplemented!();	
	}
}

