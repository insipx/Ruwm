extern crate xcb;

use ruwm::Ruwm;
mod ruwm;
mod events;
mod utils;
mod config;

fn main() {
    let ruwm = Ruwm::new().unwrap();
    ruwm.run();
}
