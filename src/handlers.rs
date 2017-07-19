use xcb;
use scancode;
use err::RuwmError;

use std::process::Command;
use xcb::ffi::base::xcb_generic_event_t as xcb_generic_event_t;

pub struct Handlers;

impl Handlers {
  pub fn handle_button_press(e: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
    let button_press : &xcb::ButtonPressEvent = xcb::cast_event(&e);
    println!("Button '{:X}' pressed", button_press.detail());
    Ok(false)
  } 
 
  pub fn handle_button_release(e: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
    let button_release : &xcb::ButtonReleaseEvent = xcb::cast_event(&e);
    println!("Button {:X} release", button_release.detail());
    Ok(false)
  }

  // Create an enum from config file of Key Combinations and their default mappings
  // that way, can just import it and do scancode::Scancode::Q.run()
  pub fn handle_key_press(e: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
    let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
    let code = scancode::Scancode::new(key_press.detail()).unwrap();
    println!("Key '{}', Hex: '{:X}' pressed, scancode: {:?}", 
      key_press.detail(), key_press.detail(), code);
    if code == scancode::Scancode::Q {
      return Ok(true);
    }
    return Ok(false);
  }

  pub fn handle_expose(e: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
    let expose : &xcb::ExposeEvent = xcb::cast_event(&e);
    println!("Received Expose Event, {:X}", expose.window());
    Ok(false)
  }

  fn handle_enter_notify(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
    unimplemented!(); 
  }

  fn event_is_ignored() -> Result<bool, RuwmError> {
    unimplemented!();
  }

  fn check_crossing_screen_boundary(x: u32, y: u32) {
    unimplemented!();
  }
}
