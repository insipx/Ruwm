pub mod parser;
use std::collections::HashMap;
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
  pub fn set(&mut self, v: String, s: Vec<String>) -> Config {
    match self.variables.contains_key(&v) {
      true => panic!("Duplicate Variable! Variable: {} already exits!", v),
      false => {
        self.variables.insert(v.clone(), s.clone());
        Config::Set(v, s) 
      }
    }
  }

  #[allow(dead_code)]
  pub fn get(&self, k: &str) -> Vec<String> {
    let result = match self.variables.get(k) {
      Some(s) => s,
      None => panic!("Variable: {} not found!", k),
    };

    return result.clone();
  }

  //if there should only be one symbol attached to a variable
  // IE for workspaces
  #[allow(dead_code)]
  pub fn get_single(&self, k: &str) -> String {
    let result = match self.variables.get(k) {
      Some(s) => s,
      None => panic!("Variable: {} not found!", k),
    };

    if result.len() > 1 {
      panic!("More than one symbol attached to this variable!: {}", k);
    }

    println!("Called!: {}", result[0]);

    String::from(result[0].clone())
  }
}

