pub mod parser;
use std::collections::HashMap;
type Direction = String;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Config {
	Set(String, Vec<String>), // variable -> Symbol
	Exec(Action),
	BindSym(Vec<String>, Action),
	FloatingMod(String),
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
pub struct Variables {
  pub variables: HashMap<String, Vec<String>>,
}

impl Variables {
  pub fn new() -> Self {

    Variables {
      variables: HashMap::new(),
    }
  }

  pub fn set(&mut self, v: String, s: Vec<String>) -> Config {
    self.variables.insert(v.clone(), s.clone());
    Config::Set(v, s)
  }

  pub fn get(&self, k: &str) -> Vec<String> {
    let result = match self.variables.get(k) {
      Some(s) => s,
      None => panic!("Variable: {} not found!", k),
    };

    return result;
  }
}

