pub mod parser;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Config {
	// Exec(Options, String),
	// Set(Variable, Symbol), // variable -> Symbol
	BindSym(Vec<String>, Action),
	FloatingMod(String),
}

/* some actions are standalone
* they can be executed without a binding
* ie: exec --no-startup-id feh ~/.config/wallpaper.png
*/
#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
	Exec(String), // standalone
	Workspace(String)
}

