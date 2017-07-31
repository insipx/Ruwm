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
// just have to make sure to free the config struct
// after the Variables struct
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum Action {
	Exec(Option<Vec<String>>, String), // standalone
	Workspace(String),
	Focus(Direction)
}


// we can just mgake the String a ref to the Vector of Symbols,
// it doesn't matter, as long as we can access those variables later.
#[derive(Debug, Clone)]
pub struct Variables {
  pub variables: HashMap<String, Vec<String>>,
}

impl Variables {

  #[allow(dead_code)]
  pub fn new() -> Self {

    Variables {
      variables: HashMap::new(),
    }
  }

  #[allow(dead_code)]
  pub fn set(&mut self, v: String, s: Vec<String>) -> Result<(), ConfigError> {
    match self.variables.contains_key(&v) {
      true => Err(ConfigError::FoundDuplicateVariable(DuplicateVariableError{v: v.to_string()})),
      false => {
        self.variables.insert(v, s);
        Ok(())
      }
    }
  }

  #[allow(dead_code)]
  pub fn get(&self, k: &String) -> Result<Vec<&str>, ConfigError> {
    match self.variables.get(k) {
      Some(s) => {
        Ok(s.iter().map(String::as_str).collect())
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: k.to_string()} )),
    }
  }

  // if there should only be one symbol attached to a variable
  // IE for workspaces
  #[allow(dead_code)]
  pub fn get_single(&self, k: &String) -> Result<&str, ConfigError> {
    match self.variables.get(k) {
      Some(s) => {
        println!("Called!: {}", s[0]);
        if s.len() > 1 {
          return Err(ConfigError::MultipleSymbols(MultipleSymbolsError {v: k.clone()}));
        }
        Ok(s[0].as_ref())
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: k.to_string()} )),
    }
  }
}

