extern crate peg;
pub mod config_parser;


// like `Ctrl+Shift+1`
type Symbol = char;
type Variable = String;

// like `move left`
type Action = String;
// like bindsym, mode, etc
type Keyword = String;
type Symbols = Vec<Symbol>;
type Actions = Vec<Actions>;

// We can expand this to make it more general
// if need be
enum Config {
	Exec(Options, String),
	// Set(Variable, Symbol), // variable -> Symbol
	BindSym(Symbols, Actions),
	FloatingMod(Symbol),
}
