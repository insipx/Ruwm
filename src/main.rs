extern crate xcb;
extern crate colored;

use ruwm::Ruwm;

mod ruwm;
mod handlers;
mod utils;
mod config;
mod err;
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
