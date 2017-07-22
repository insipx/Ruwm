extern crate peg;
pub mod config_parser;

pub struct ConfigParser;

enum BindSym {
	Key(String),
	Symbols(String),
	Actions(Vec<String>),
}

enum Symbols {
	
}

enum Variables {

	
}

peg! parser(r#"
#[pub]


	"#)