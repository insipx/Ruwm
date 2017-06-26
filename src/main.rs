extern crate xcb;

use ruwm::Ruwm;
mod ruwm;

fn main() {
    let dpy = ":1";
    let ruwm = Ruwm::new(&dpy).unwrap();
    ruwm.run();
}
