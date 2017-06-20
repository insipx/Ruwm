extern crate xcb;

use ruwm::Ruwm;
mod ruwm;

fn main() {
    let ruwm = Ruwm::new().unwrap();
    ruwm.run();
}
