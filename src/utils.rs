extern crate xcb;
use std;
use xcb::ffi::xproto::xcb_atom_t as xcb_atom_t;
struct Utils;

impl Utils {
  pub fn get_atoms<'a>(names: Vec<&str>, atoms: Vec<xcb_atom_t>, c: &xcb::Connection) {
    let cookies: <std::vec::Vec<xcb::Cookie<'a, xcb::ffi::xcb_intern_atom_cookie_t>> as Trait>::new();
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