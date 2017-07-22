extern crate xcb;
use self::xcb::ffi::xproto::xcb_atom_t as xcb_atom_t;
use self::xcb::ffi::xproto::xcb_intern_atom_cookie_t as xcb_intern_atom_cookie_t;
pub struct Utils;

impl Utils {
  pub fn get_atoms<'a>(names: Vec<&str>, atoms: &mut Vec<xcb_atom_t>, c: &xcb::Connection) {

    type CookieAtom<'a> = xcb::Cookie<'a, xcb_intern_atom_cookie_t>;
    let mut cookies = Vec::<CookieAtom>::new();

    for name in &names {
      cookies.push(xcb::intern_atom(c, false, name));
    }

    for (name, cookie) in names.iter().zip(cookies.iter()) {
      let reply = cookie.get_reply();
      match reply {
        Ok(x) => atoms.push(x.atom()),
        Err(e) => panic!("Error! in get_atoms(), error: {:?}", e),
      }
    }
  }
}