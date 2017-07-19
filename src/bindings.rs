use err::RuwmError;

use xcb;
use std::collections::HashMap;

// not sure if this will work
use scancode::Scancode;

// A Scancode or combination of Scancodes
// Which dictate an action
#[derive(PartialEq, Eq, Hash)]
struct Binding {
	// Vector of scancodes
	// if a binding is only one keystroke
	// multiple 
	keys: Vec<Scancode>,
}
#[derive(PartialEq, Eq, Hash)]
struct Action {
	// TODO
	// Figure out how to represent Actions
	placeholder: i32,	
}

pub struct Bindings {
	// Scancode/Scancode Combo => Action
	pub bindings: HashMap<Binding, Action>,
}

impl Bindings {
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

