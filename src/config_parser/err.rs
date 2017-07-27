extern crate peg;

use std::io;
use std::error; 
use std::fmt;

use self::peg::grammar::ParseError;

use config_parser::colored::Colorize;

/* 
 * Need to eventually impelement std::Error trait
 * for err.rs
 *
 */
// v = variable
#[derive(Debug)]
struct VariableNotFound { v: String }

#[derive(Debug)]
struct DuplicateVariable { v: String }

#[derive(Debug)]
struct MultipleSymbols { v: String }

pub type VariableNotFoundError = VariableNotFound;

impl fmt::Display for VariableNotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Variable Not Found") 
  }
}

impl error::Error for VariableNotFoundError {
  fn description(&self) -> &str {
    "The Variable Could Not Be Found, Make Sure It is Defined in the Configuration File" 
  }
}

pub type DuplicateVariableError = DuplicateVariable;
impl fmt::Display for DuplicateVariableError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Duplicate Variable") 
  } 
}

impl error::Error for DuplicateVariableError {
  fn description(&self) -> &str {
    "A Duplicate Error Was Found In the Configuration File; \
     Make Sure that Only One Variable \
     is Defined per Variable Name" 
  } 
}

pub type MultipleSymbolsError = MultipleSymbols;
impl fmt::Display for MultipleSymbolsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "More Than One Symbol") 
  }
}

impl error::Error for MultipleSymbolsError {
  fn description(&self) -> &str {
    "More than One Symbol Attached to Variable \
    For some config directives (like 'workspace') only \
    one symbol may be associated with a variable"
  }
}


#[derive(Debug)]
pub enum ConfigError {
  // If file open fails
  CouldNotOpenConfigFile(io::Error),
  // If the grammar fails
  CouldNotParseConfigFile(ParseError), 
  // If a duplicate variable is found in the config
  FoundDuplicateVariable(DuplicateVariableError), // String is the variable
  // If a variable is not found in config
  VariableNotFound(VariableNotFoundError), // String is the variable
  // If a variable is attached to more than one symbol,
  // when only one symbol is expected
  MoreThanOneSymbolAttachedToVariable(MultipleSymbolsError), // String is the variable
}


impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConfigError::CouldNotOpenConfigFile(ref err) => write!(f, "Could Not Open Config File {}", err),
      ConfigError::CouldNotParseConfigFile(ref err) => write!(f, "Could Not Parse Config File {}", err),
      ConfigError::FoundDuplicateVariable(ref err) => write!(f, "Found Duplicate Variable {}", err),
      ConfigError::VariableNotFound(ref err) => write!(f, "Variable '{}' Not Found ", err),
      ConfigError::MoreThanOneSymbolAttachedToVariable(ref err) => write!(f, "More Than One Symbol Attached to Variable '{}'", err),      
    }
  }
}

impl error::Error for ConfigError {
  fn description(&self) -> &str {
    match *self {
      ConfigError::CouldNotOpenConfigFile(ref err) => err.description(),
      ConfigError::CouldNotParseConfigFile(ref err) => err.description(),
      ConfigError::FoundDuplicateVariable(ref err) => err.description(),
      ConfigError::VariableNotFound(ref err) => err.description(),
      ConfigError::MoreThanOneSymbolAttachedToVariable(ref err) => err.description(), 
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      ConfigError::CouldNotOpenConfigFile(ref err) => Some(err),
      ConfigError::CouldNotParseConfigFile(ref err) => Some(err),
      ConfigError::FoundDuplicateVariable(ref err) => Some(err),
      ConfigError::VariableNotFound(ref err) => Some(err),
      ConfigError::MoreThanOneSymbolAttachedToVariable(ref err) => Some(err) 
    }
  }
}

// From Conversions
impl From<io::Error> for ConfigError {
  fn from(err: io::Error ) -> ConfigError {
    ConfigError::CouldNotOpenConfigFile(err)
  }
}

impl From<ParseError> for ConfigError {
  fn from(err: ParseError) -> ConfigError {
    ConfigError::CouldNotParseConfigFile(err)
  }
}

impl From<DuplicateVariableError> for ConfigError {
  fn from(err: DuplicateVariableError) -> ConfigError {
    ConfigError::FoundDuplicateVariable(err)
  }
}

impl From<VariableNotFoundError> for ConfigError {
  fn from(err: VariableNotFoundError) -> ConfigError {
    ConfigError::VariableNotFound(err)
  }
}

impl From<MultipleSymbolsError> for ConfigError {
  fn from(err: MultipleSymbolsError) -> ConfigError {
    ConfigError::MoreThanOneSymbolAttachedToVariable(err)
  }
}
