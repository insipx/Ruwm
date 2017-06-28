extern crate xcb;

use ruwm::Ruwm;
mod ruwm;
mod events;
mod utils;

fn main() {
    let ruwm = Ruwm::new().unwrap();
    ruwm.run();
}
