pub mod parser;

#[derive(Debug)]
pub enum Config {
	// Exec(Options, String),
	// Set(Variable, Symbol), // variable -> Symbol
	BindSym(Vec<String>, Vec<String>),
	FloatingMod(String),
}


