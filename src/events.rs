extern crate xcb;
extern crate scancode;

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

  // Create an enum from config file of Key Combinations and their default mappings
  // that way, can just import it and do scancode::Scancode::Q.run()
  pub fn key_press(e: xcb::Event<xcb_generic_event_t>) -> bool {
    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
    let code = scancode::Scancode::new(key_press.detail()).unwrap();
    println!("Key '{}', Hex: '{:X}' pressed, scancode: {:?}", 
      key_press.detail(), key_press.detail(), code);
    if code == scancode::Scancode::Q {
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