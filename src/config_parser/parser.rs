use std::io;
use std::io::prelude::*;
use std::fs::File;

use self::err::ConfigError;
/* includes the generated code from PEG 
	- ${PROJECT_DIR}/targetbuild/ruwm-*out/
*/

use config_parser::*;

#[allow(dead_code)]
pub mod config_grammar {
	include!(concat!(env!("OUT_DIR"), "/config_grammar.rs"));
}

pub struct Parser<'a> {
  variables: Variables<'a>,
  config: Vec<Config>,
}

impl<'a> Parser<'a> {

  pub fn new(config_file: &'a str) -> Result<Parser, ConfigError>  {
    let mut f = File::open(config_file)?;
    let mut config = String::new();
    f.read_to_string(&mut config);

    Ok(Parser {
      variables: Variables::new(),
      config: match config_grammar::content(&config) {
        Ok(c) => c,
        Err(e) => panic!("Could not parse config"),
      },
    })
  }

  pub fn parse(&mut self) {
    unimplemented!();
  }

  /*
   * Put variables (anything starting with $) in a hashmap
   */
  fn intern_variables() {
    unimplemented!();
  }
}



#[cfg(test)]
#[test]
fn test_grammar() {
  // TODO
  // the fact that the first line must be at the very beginning
  // is not a very big deal, since we can just strip all whitespace until
  // we reach an ascii character
  // but if there is a easy way to fix this in the grammar, then that is the
  // better option
	match config_grammar::content("# A comment
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

"){
		Ok(r) => println!("Parsed as: {:?}", r),
		Err(e) => println!("Parse error: {}", e),
	};

}

#[cfg(test)]
#[test]
fn test_parser() {
  println!("Test for the Parser Struct and functions go here");
}
