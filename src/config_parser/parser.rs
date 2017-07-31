use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

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
pub struct Parser<'a> {
  variables: Variables<'a>,
  bindSym:  Vec<Config>,
  exec: Vec<Config>,
  floatingMod: Vec<Config>,
}

impl<'a> fmt::Display for Parser<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Variables: {:?}, \
      bindSym: {:?}, \
      exec: {:?}, \
      floatingMod: {:?}", 
      self.variables, self.bindSym, self.exec, self.floatingMod
    )
  }
}

impl<'a> Parser<'a> {

  pub fn new(config_file: &'a str) -> Result<Parser, ConfigError>  {
    let mut f = File::open(config_file)?;
/*    {
      let mut test = String::new();
      f.read_to_string(&mut test)?;
      println!("CONFIG: {}", test);
    }*/

    let result = Self::parse(&mut f)?;
    Ok(Parser {
      variables: result.0,
      bindSym: result.1,
      exec: result.2,
      floatingMod: result.3,
    })
  }

  /*
   * Parse configuration file, interning Variables,
   * and setting up the Parser Struct
   */
  fn parse(mut file: &mut File)
  -> Result<(Variables<'a>, Vec<Config>, Vec<Config>, Vec<Config>), ConfigError> 
  {
    let mut config = String::new();
    // println!("first config, empty: {}", config);
    file.read_to_string(&mut config)?;
    println!("Should not be empty: {}", config);
    let config = config_grammar::content(config.as_ref())?;

    // create data structs for each configuration type
    let mut variables = Variables::new();
    let mut bindSym = Vec::new();
    let mut exec = Vec::new();
    let mut floatingMod = Vec::new();

    for x in config.iter() {
      match *x {
        Config::Set(ref v, ref s) => variables.set(v.to_string(), s.to_owned().iter().map(|s| s: &'a String).collect())?,
        Config::Exec(ref a) => exec.push(Config::Exec(a.to_owned())),
        Config::BindSym(ref s, ref a) => bindSym.push(Config::BindSym(s.to_owned(),a.to_owned())),
        Config::FloatingMod(ref s) => floatingMod.push(Config::FloatingMod(s.to_owned())),
        _ => {},
      };
    }
    Ok((variables, bindSym, exec, floatingMod))
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
  let parser = match Parser::new("/home/insi/Projects/ruwm/src/config_parser/config.test") {
    Ok(s) => s,
    Err(e) => panic!("{}", e)
  };

  println!("Parser: {}", parser.to_string());
}
