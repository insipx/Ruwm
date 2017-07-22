/* includes the generated code from PEG 
	- ${PROJECT_DIR}/targetbuild/ruwm-*out/
*/

use config_parser::Config;
use config_parser::Action;

pub mod config_grammar {
	include!(concat!(env!("OUT_DIR"), "/config_grammar.rs"));
}

#[cfg(test)]
#[test]
fn test_parser() {
	match config_grammar::content("bindsym LeftGui+1 workspace 1 
   
   bindsym RightGui+2 workspace 2       


bindsym Hello+3 workspace 5


"){
		Ok(r) => println!("Parsed as: {:?}", r),
		Err(e) => println!("Parse error: {}", e),
	}	
}