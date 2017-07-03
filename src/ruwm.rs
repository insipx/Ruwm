extern crate xcb;

use events::Events;
use utils::Utils;
use config::WM_ATOM_NAME as WM_ATOM_NAME;
use config::NET_ATOM_NAME as NET_ATOM_NAME;

use xcb::ffi::xproto::xcb_atom_t as xcb_atom_t;

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

  /* setup a new instance of Ruwm and set it up */

  pub fn new() -> Option<Self> {
    let mut wmatoms : Vec<xcb_atom_t> = Vec::new();
    let mut netatoms : Vec<xcb_atom_t> = Vec::new();

    let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

    println!("Screen Num: {}", screen_num);
    let setup = Self::setup(&connection, screen_num);
    Utils::get_atoms(WM_ATOM_NAME.to_vec(), &mut wmatoms, &connection);
    Utils::get_atoms(NET_ATOM_NAME.to_vec(), &mut netatoms, &connection);
    println!("ATOM WM: {:?}", wmatoms);
    println!("ATOM NET: {:?}", netatoms);

    Some( Ruwm{
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
  pub fn setup(ref connection: &xcb::Connection, screen_num: i32) -> (u16, u16, u32) {
    let setup = connection.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    return (screen.width_in_pixels(), screen.height_in_pixels(), screen.root());
  }

  pub fn run(&self) {
    let events = [(
      xcb::CW_EVENT_MASK,
      xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE | xcb::EVENT_MASK_KEY_PRESS |
       xcb::EVENT_MASK_EXPOSURE
    )];

    xcb::change_window_attributes(&self.connection, self.root, &events);

    self.connection.flush();

    'event_loop : loop {
      let event = self.connection.wait_for_event();
      match event {
        Some(e) => {
          match e.response_type() {
            xcb::BUTTON_PRESS => Events::button_press(e),
            xcb::BUTTON_RELEASE => Events::button_release(e),
            xcb::KEY_PRESS => {
              let res = Events::key_press(e);
              if res {
                break 'event_loop;
              }
            },
            xcb::EXPOSE => Events::expose(e),
            _ => { 
              println!("Received some event: {}", e.response_type());
            }
          }
        },
        None => { }
      }
    }
  }
}
