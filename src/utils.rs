extern crate xcb;
use std;
use xcb::ffi::xproto::xcb_atom_t as xcb_atom_t;
struct Utils;

impl Utils {
  pub fn get_atoms(names: Vec<&str>, atoms: Vec<xcb_atom_t>, c: &xcb::Connection) {
    type AtomicCookie<'a> = xcb::Cookie<'a, xcb::ffi::xproto::xcb_intern_atom_cookie_t>;
    let cookies : std::vec::Vec<AtomicCookie>::new();

    for name in &names {
      cookies.push(xcb::intern_atom(c, false, name));
    }

    for (name, cookie) in names.iter().zip(cookies.iter()) {
      let reply = cookie.get_reply();
      match reply {
        Ok(x) => atoms.push(x.atom()),
        Err(e) => panic!("Error! {:?}", e),
      }
    }
  }
}