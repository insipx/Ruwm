/* includes the generated code from PEG 
	- ${PROJECT_DIR}/targetbuild/ruwm-*out/
*/

use config_parser::*;

pub mod config_grammar {
	include!(concat!(env!("OUT_DIR"), "/config_grammar.rs"));
}

#[cfg(test)]
#[test]
fn test_parser() {

  let mut variables = Variables::new();
  // TODO
  // the fact that the first line must be at the very beginning
  // is not a very big deal, since we can just strip all whitespace until
  // we reach an ascii character
  // but if there is a easy way to fix this in the grammar, then that is the
  // better option
	match config_grammar::content("bindsym LeftGui+1 workspace 1 

bindsym $super+$rand exec 'termite --config $HOME/.config/termite/config'
   bindsym RightGui+2 workspace 2 # this is another comment


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

", &mut variables){
		Ok(r) => println!("Parsed as: {:?}", r),
		Err(e) => println!("Parse error: {}", e),
	}	
}
