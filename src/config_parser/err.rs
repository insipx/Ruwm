use config_parser::colored::Colorize;
/* 
 * Need to eventually impelement std::Error trait
 * for err.rs
 *
 * THis is just a prototype for use until i figure out errors
 */


#[derive(Debug)]
pub enum ConfigError {
  // If file open fails
  CouldNotOpenConfigFile(),
  // If the grammar fails
  CouldNotParseConfigFile(), 
  // If a duplicate variable is found in the config
  FoundDuplicateVariable(String),
  // If a variable is not found in config
  VariableNotFound(String), 
  // If a variable is attached to more than one symbol,
  // when only one symbol is expected
  MoreThanOneSymbolAttachedToVariable(String),
}
