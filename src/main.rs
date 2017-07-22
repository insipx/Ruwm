mod ruwm;
mod config_parser;
use config_parser::parser::config_grammar as parser;

use ruwm::ruwm::Ruwm;
// GENERAL
// TODO
// Create a WmError Struct
fn main() {
    // argparser goes here
    // handles passing configuration to Ruwm Struct
    // Starts IPC

    let ruwm = Ruwm::new().unwrap();
    ruwm.run();
}


