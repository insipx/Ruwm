extern crate xcb;

use std::process::Command;

use xcb::ffi::base::xcb_generic_event_t as xcb_generic_event_t;

pub struct Events;

impl Events {
  pub fn button_press(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let button_press : &xcb::ButtonPressEvent = xcb::cast_event(&e);
    println!("Button '{:X}' pressed", button_press.detail());
    false
  } 
 
  pub fn button_release(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let button_release : &xcb::ButtonReleaseEvent = xcb::cast_event(&e);
    println!("Button {:X} release", button_release.detail());
    false
  }

  pub fn key_press(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
    println!("Key '{}', Hex: '{:X}' pressed", 
      key_press.detail(), key_press.detail());
    if key_press.detail() == 0x18 {
      return true;
    }
    return false;
  }

  pub fn expose(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let expose : &xcb::ExposeEvent = xcb::cast_event(&e);
    println!("Received Expose Event, {:X}", expose.window());
    false
  }

}