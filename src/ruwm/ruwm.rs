use ruwm::xcb;
use std::process;

use ruwm::err::RuwmError;
use ruwm::handlers::Handlers;
use ruwm::utils::Utils;
use ruwm::config::WM_ATOM_NAME as WM_ATOM_NAME; 
use ruwm::config::NET_ATOM_NAME as NET_ATOM_NAME;


use self::xcb::ffi::xproto::xcb_atom_t as xcb_atom_t;

// Handles setup and the event loop (running)
pub struct Ruwm {
  width: u16,
  height: u16,
  root: u32,
  connection: xcb::Connection,
  screen_num: i32,
  wmatoms: Vec<xcb_atom_t>,
  netatoms: Vec<xcb_atom_t>,
}

impl Ruwm {

  /* create a new instance of Ruwm and set it up */
  /* get a connection to xserver, create a new instance of Ruwm and set it up */
  /* Parse any configuration from config file */

  pub fn new() -> Result<Ruwm, RuwmError> {
    let mut wmatoms : Vec<xcb_atom_t> = Vec::new();
    let mut netatoms : Vec<xcb_atom_t> = Vec::new();

    // make a connection to the X server
    let (connection, screen_num) = match xcb::Connection::connect(None) {
      Ok(c) => c,
      Err(e) => RuwmError::CouldNotConnect(e).handle(),
    };

    // setup roots, width and height
    let setup = match Self::setup(&connection, screen_num) {
      Ok(s) => s,
      Err(e) => e.handle(),
    };

    println!("Screen Num: {}", screen_num);
    Utils::get_atoms(WM_ATOM_NAME.to_vec(), &mut wmatoms, &connection);
    Utils::get_atoms(NET_ATOM_NAME.to_vec(), &mut netatoms, &connection);
    println!("ATOM WM: {:?}", wmatoms);
    println!("ATOM NET: {:?}", netatoms);

    Ok( Ruwm{
      width: setup.0,
      height: setup.1,
      root: setup.2,
      connection,
      screen_num,
      wmatoms,
      netatoms,
    })
  }
  
  /* get width+height and the root screen */  
  pub fn setup(ref connection: &xcb::Connection, screen_num: i32)
  -> Result<(u16, u16, u32), RuwmError> {
    let setup = connection.get_setup();
    if let Some(screen) = setup.roots().nth(screen_num as usize) {
      Ok((screen.width_in_pixels(), screen.height_in_pixels(), screen.root()))
    } else {
      Err(RuwmError::CouldNotAcquireScreen)
    }
  }

  pub fn run(&self) { 
    let events = [(
      xcb::CW_EVENT_MASK,
      xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE |
      xcb::EVENT_MASK_KEY_PRESS | xcb::EVENT_MASK_EXPOSURE |
      xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT | xcb::EVENT_MASK_SUBSTRUCTURE_NOTIFY
    )];

    let cookie = xcb::change_window_attributes(&self.connection, self.root, &events);

    if ! cookie.request_check().is_ok() {
      panic!("There's another Window Manager Running!");
    }

    self.connection.flush();

    'event_loop : loop {
      let event = self.connection.wait_for_event();
      match event {
        Some(e) => {
          let result = match e.response_type() {
            xcb::BUTTON_PRESS => Handlers::handle_button_press(e),
            xcb::BUTTON_RELEASE => Handlers::handle_button_release(e),
            xcb::KEY_PRESS => Handlers::handle_key_press(e), 
            xcb::EXPOSE => Handlers::handle_expose(e),
            xcb::MAP_REQUEST => Handlers::handle_map_request(e),
            _ => {
              println!("Received some event: {}", e.response_type());
              Ok(false)
            }
          };
          match result {
            Ok(r) => { if r { break 'event_loop; } },
            Err(e) => { e.handle() }
          }
        },
        None => { }
      };
    }
    println!("Exiting!");
  }
}
