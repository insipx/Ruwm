extern crate colored;

pub mod parser; 
pub mod err;

use std::collections::HashMap;
use self::err::*;

type Direction = String;

/* 
 * For some reason, the rustc compiler thinks this is all dead code
 * probably since it's attached to the compiler, and the code that uses
 * these structs is generated, the compiler does not pick up on this
 * that is why this is all marked as allow(dead_code)
 */

#[derive(Debug)]
#[allow(dead_code)]
pub enum Config {
	Set(String, Vec<String>), // variable -> Symbol
	Exec(Action),
	BindSym(Vec<String>, Action),
	FloatingMod(String),
  Comment()
}

/* some actions are standalone
* they can be executed without a binding
* ie: exec --no-startup-id feh ~/.config/wallpaper.png
*/
#[allow(dead_code)]
#[derive(Debug)]
pub enum Action {
	Exec(Option<Vec<String>>, String), // standalone
	Workspace(String),
	Focus(Direction)
}


// we can just make the String a ref to the Vector of Symbols,
// it doesn't matter, as long as we can access those variables later.
pub struct Variables<'a> {
  pub variables: HashMap<String, Vec<&'a str>>,
}

impl<'a> Variables<'a> {
  #[allow(dead_code)]
  pub fn new() -> Self {

    Variables {
      variables: HashMap::new(),
    }
  }

  #[allow(dead_code)]
  pub fn set(&mut self, v: String, s: Vec<&'a str>) -> Result<(), ConfigError> {
    match self.variables.contains_key(&v) {
      true => Err(ConfigError::FoundDuplicateVariable(DuplicateVariableError{v})),
      false => {
        self.variables.insert(v, s);
        Ok(())
      }
    }
  }

  #[allow(dead_code)]
  pub fn get(&'a self, k: &str) -> Result<Vec<&'a str>, ConfigError> {
    match self.variables.get(k) {
      Some(s) => {
        Ok(s.to_owned())
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: String::from(k)} )),
    }
  }

  // if there should only be one symbol attached to a variable
  // IE for workspaces
  #[allow(dead_code)]
  pub fn get_single(&'a self, k: &str) -> Result<&'a str, ConfigError> {
    return match self.variables.get(k) {
      Some(s) => {
        println!("Called!: {}", s[0]);
        if s.len() > 1 {
          return Err(ConfigError::MoreThanOneSymbolAttachedToVariable(MultipleSymbolsError {v: String::from(k)}));
        }
        Ok(s[0].as_ref())
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: String::from(k)} )),
    };
  }
}

