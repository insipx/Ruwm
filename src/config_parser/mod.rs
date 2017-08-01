extern crate colored;

pub mod parser; 
pub mod err;

use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

use self::err::*;

/* 
 * For some reason, the rustc compiler thinks this is all dead code
 * probably since it's attached to the compiler, and the code that uses
 * these structs is generated, the compiler does not pick up on this
 * that is why this is all marked as allow(dead_code)
 */
 #[derive(Debug, Clone)]
pub enum Value {
  Literal(String),
  VariableReference(String),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Value::Literal(ref s) => write!(f, "Literal: {}", s),
      Value::VariableReference(ref s) => write!(f, "VariableReference: {}", s)
    }
  }
}

impl Into<String> for Value {
  fn into(self) -> String {
    match self {
      Value::Literal(s) => s,
      Value::VariableReference(s) => s, 
    }
  }
}

// just have to make sure to free the config struct
// after the Variables struct
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Config {
	Set(String /* var name */, Vec<Value>), // variable -> Symbol
	Exec(Action),
	BindSym(Vec<Value>, Action),
	FloatingMod(Value),
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
	Workspace(Value),
	Focus(Value)
}


// we can just mgake the String a ref to the Vector of Symbols,
// it doesn't matter, as long as we can access those variables later.
#[derive(Debug, Clone)]
pub struct Variables(HashMap<String, Vec<Value>>);

impl Deref for Variables {
  type Target = HashMap<String, Vec<Value>>;
  fn deref(&self) -> &HashMap<String, Vec<Value>> {
    &self.0
  }
}

impl Variables {

  #[allow(dead_code)]
  pub fn new() -> Self {
    Variables { 0: HashMap::new() }
  }

  #[allow(dead_code)]
  pub fn set(&mut self, v: String, s: Vec<Value>) -> Result<(), ConfigError> {
    match self.0.contains_key(&v) {
      true => Err(ConfigError::FoundDuplicateVariable(DuplicateVariableError{v: v.to_string()})),
      false => {
        // TODO this will make Variables in the Hashmap as Symbols
        // need to fix that
        // let s = s.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        self.0.insert(v, s);
        Ok(())
      }
    }
  }

  #[allow(dead_code)]
  // useds recursion if a 'set' references another variable
  pub fn get(&self, k: &String) -> Result<Vec<&str>, ConfigError> {
    match self.0.get(k) {
      Some(s) => {
        let mut result: Vec<&str> = Vec::new();
        for v in s.iter() {
          match *v {
            Value::Literal(ref l) => result.push(l),
            Value::VariableReference(ref r) => result.extend(self.get(r)?),
          }
        }
        Ok(result)
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: k.to_string()} )),
    }
  }

  // if there should only be one symbol attached to a variable
  // IE for workspaces
  #[allow(dead_code)]
  pub fn get_single(&self, k: &String) -> Result<&str, ConfigError> {
    match self.0.get(k) {
      Some(s) => {
        if s.len() > 1 {
          return Err(ConfigError::MultipleSymbols(MultipleSymbolsError {v: k.clone()}));
        }
        match s[0] {
          Value::Literal(ref l) => Ok(l),
          Value::VariableReference(ref r) => self.get_single(r)
        }
      },
      None => Err(ConfigError::VariableNotFound(VariableNotFoundError{ v: k.to_string()} )),
    }
  }
}

