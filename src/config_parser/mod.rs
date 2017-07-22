extern crate peg;
pub mod config_parser;

pub struct ConfigParser;

// like `move left`
type Action = String;
// like `Ctrl+Shift+1`
type Symbol = char;
// like bindsym, mode, etc
type Keyword = String;

type Symbols = Vec<Symbol>;
type Actions = Vec<Actions>;

// We can expand this to make it more general
// if need be
enum BindSym {
	Symbols(Symbols),
	Actions(Actions),
}

peg! parser(r#"
#[pub]

keyword -> super::Keyword
	= 

symbol -> super::Symbol
	=

action -> super::Action
	= 

symbols -> super::Symbols
	=

actions -> super::Actions
	= 

/* BindSym, all we care about is what symbols 
 * are associated with which actions
 */
bindsym -> super::BindSym
	= k:keyword s:symbols a:actions { super::BindSym(s,a)}


	"#)

#[test]
fn test() {

}