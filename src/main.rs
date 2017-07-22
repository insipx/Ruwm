mod ruwm;

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
