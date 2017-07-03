extern crate xcb;
use xcb::ffi::base::xcb_generic_event_t as xcb_generic_event_t;
pub struct Events;

impl Events {
  pub fn button_press(e: xcb::Event<xcb_generic_event_t>) {
    let button_press : &xcb::ButtonPressEvent = xcb::cast_event(&e);
    println!("Something Happened!, '{}' pressed", button_press.detail());
  } 
 
  pub fn button_release(e: xcb::Event<xcb_generic_event_t>) {
    let button_release : &xcb::ButtonReleaseEvent = xcb::cast_event(&e);
    println!("Button Release: {:?}", button_release.detail());
  }

  pub fn key_press(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
    println!("Key '{}' pressed", key_press.detail());
    if key_press.detail() == 0x18 {
      return true
    } else {
      return false;
    }
  }

  pub fn expose(e: xcb::Event<xcb_generic_event_t>) {
    let expose : &xcb::ExposeEvent = xcb::cast_event(&e);
    println!("Received Expose Event, {}", expose.window());
  }

}