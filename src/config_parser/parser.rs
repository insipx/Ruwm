use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::error::Error;

use self::err::ConfigError;
/* includes the generated code from PEG 
	- ${PROJECT_DIR}/targetbuild/ruwm-*out/
*/
// TODO
// This code need some major cleanup

use config_parser::*;

#[allow(dead_code)]
pub mod config_grammar {
    include!(concat!(env!("OUT_DIR"), "/config_grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
    vars: Variables,
    bind_sym: Vec<Config>,
    exec: Vec<Config>,
    floating_mod: Vec<Config>,
}

/*
 * Set up the Parser Struct
 * move ownership of the parsed file from
 * the rust-peg grammar (rust-peg) to my own structures
 * Set Keybindings
 * Handle Reloads
 *
 */
impl Parser {
    #[allow(dead_code)]
    pub fn new(config_file: &str) -> Result<Parser, ConfigError> {
        let mut f = File::open(config_file)?;

        let mut config = String::new();
        f.read_to_string(&mut config)?;
        let config = config_grammar::content(config.as_ref())?;

        // create data structs for each configuration type
        let mut vars = Variables::new();
        let mut bind_sym = Vec::new();
        let mut exec = Vec::new();
        let mut floating_mod = Vec::new();

        for x in config.iter() {
            match *x {
                Config::Set(ref v, ref s) => vars.set(v.to_owned(), s.to_owned())?,
                Config::Exec(ref a) => exec.push(Config::Exec(a.to_owned())),
                Config::BindSym(ref s, ref a) => {
                    bind_sym.push(Config::BindSym(s.to_owned(), a.to_owned()))
                }
                Config::FloatingMod(ref s) => floating_mod.push(Config::FloatingMod(s.to_owned())),
                _ => {}
            };
        }

        Ok(Parser {
            vars,
            bind_sym,
            exec,
            floating_mod,
        })
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n
    {:?}, \n
	{:?}, \n
    {:?}, \n
    {:?}  \n",
            self.vars,
            self.bind_sym,
            self.exec,
            self.floating_mod
        )
    }
}

#[cfg(test)]
#[test]
fn test_grammar() {
    // banner for --nocapture
    // TODO
    // the fact that the first line must be at the very beginning
    // is not a very big deal, since we can just strip all whitespace until
    // we reach an ascii character
    // but if there is a easy way to fix this in the grammar, then that is the
    // better option
    match config_grammar::content(
        "# A comment
		bindsym LeftGui+1 workspace 1 

bindsym $super+$rand exec 'termite --config $HOME/.config/termite/config'
   bindsym RightGui+2 workspace 2 # this is another comment

set $super Ctrl+Mod4
  # This is a comment
set $ws0 '0:emp'
bindsym RightGui+3 workspace $ws0
bindsym Hello+3 workspace 5

bindsym $super+$left focus left
bindsym $super+Ctrl focus down
bindsym $super+$up focus up
bindsym $super+$right focus right
bindsym $super+1 workspace $ws0

exec --no-startup-id 'nm-applet'

set $left h

",
    ) {
        Ok(r) => println!("Parsed as: {:?}", r),
        Err(e) => println!("Parse error: {}", e),
    };

}

#[cfg(test)]
#[test]
fn test_parser() {
    // banner for --nocapture
    let parser = match Parser::new("src/config_parser/config.test") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    match parser.vars.get(&"$longchain".to_string()) {
        Ok(s) => {
            println!("Long Chain: {:?}", s);
            assert_eq!(s, vec!["l"]);
        }
        Err(e) => panic!("{}", e),
    };

    match parser.vars.get(&"$long_ordered_chain".to_string()) {
        Ok(s) => {
            println!("Long Ordered Chain: {:?}", s);
            assert_eq!(s, vec!["a", "b", "l", "Mod3", "f", "h", "z"]);
        }
        Err(e) => panic!("{}", e),
    };
    println!("Parser: {}", parser.to_string());
}
