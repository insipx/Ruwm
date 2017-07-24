mod ruwm;
mod config_parser;
use config_parser::parser::config_grammar as parser;

/*
 * For general todo/plan go to plan/plan.md
 * Main should do a few things - but nothing too specific
 * mostly it manages the abstract objects
 * Ruwm, IPC, Argparser, Setting up the configuration
 * Everything else happens in Ruwm or libraries
 */
>>>>>>> Stashed changes
fn main() {
    // argparser goes here
    // handles passing configuration to Ruwm Struct
    // Starts IPC

    let ruwm = Ruwm::new().unwrap();
    ruwm.run();
}


