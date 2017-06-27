extern crate xcb;
pub struct Ruwm {
  width: u16,
  height: u16,
  root: u32,
  connection: xcb::Connection,
  screen_num: i32,
}

impl Ruwm {

  /* setup a new instance of Ruwm and set it up */

  pub fn new() -> Option<Self> {
    let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

    println!("Screen Num: {}", screen_num);
    let setup = Self::setup(&connection, screen_num);

    Some( Ruwm{
      width: setup.0,
      height: setup.1,
      root: setup.2,
      connection,
      screen_num,
    })
  }
  
  /* get width+height and the root screen */  
  pub fn setup(ref connection: &xcb::Connection, screen_num: i32) -> (u16, u16, u32) {
    let setup = connection.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    return (screen.width_in_pixels(), screen.height_in_pixels(), screen.root());
  }

  pub fn run(&self) {

    /*for x in setup.roots() {
      unsafe {
        println!("x: {:?}", (*x.ptr).root);
        println!("x: {:?}", (*x.ptr).default_colormap);
        println!("x: {:?}", (*x.ptr).white_pixel);
        println!("x: {:?}", (*x.ptr).black_pixel);
        println!("x: {:?}", (*x.ptr).current_input_masks);
        println!("x: {:?}", (*x.ptr).width_in_pixels);
        println!("x: {:?}", (*x.ptr).height_in_pixels);
        println!("x: {:?}", (*x.ptr).width_in_millimeters);
        println!("x: {:?}", (*x.ptr).height_in_millimeters);
        println!("x: {:?}", (*x.ptr).min_installed_maps);
        println!("x: {:?}", (*x.ptr).max_installed_maps);
        println!("x: {:?}", (*x.ptr).root_visual);
        println!("x: {:?}", (*x.ptr).backing_stores);
        println!("x: {:?}", (*x.ptr).save_unders);
        println!("x: {:?}", (*x.ptr).root_depth);
        println!("x: {:?}", (*x.ptr).allowed_depths_len);
      }
    }*/

    println!("Screen height: {}", self.height);
    println!("Screen width: {}", self.width);

    let window = self.connection.generate_id();
    let events : u16 = 
      (xcb::CW_EVENT_MASK | xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE | xcb::EVENT_MASK_KEY_PRESS | xcb::EVENT_MASK_EXPOSURE) as u16;

    xcb::grab_key(&self.connection, true, self.root, xcb::MOD_MASK_2 as u16, 
      xcb::NO_SYMBOL as u8, xcb::GRAB_MODE_ASYNC as u8, xcb::GRAB_MODE_ASYNC as u8);
    
    xcb::grab_button(&self.connection, false, self.root, events, xcb::GRAB_MODE_ASYNC as u8, 
      xcb::GRAB_MODE_ASYNC as u8, self.root, xcb::NONE, 1, xcb::MOD_MASK_1 as u16);
  
    xcb::grab_button(&self.connection, false, self.root, events, xcb::GRAB_MODE_ASYNC as u8,
      xcb::GRAB_MODE_ASYNC as u8, self.root, xcb::NONE, 3, xcb::MOD_MASK_1 as u16);

    self.connection.flush();

    'event_loop : loop {
      let event = self.connection.wait_for_event();
      match event {
        Some(e) => {
          match e.response_type() {
            xcb::BUTTON_PRESS => {
              let button_press : &xcb::ButtonPressEvent = xcb::cast_event(&e);
              println!("Something Happened!, '{}' pressed", button_press.detail());
            },
            xcb::BUTTON_RELEASE => {

            },
            xcb::KEY_PRESS => {
              let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
              println!("Key '{}' pressed", key_press.detail());
              if key_press.detail() == 0x18 { //Q
                break 'event_loop;
              }
            },
            xcb::EXPOSE => {
              let expose : &xcb::ExposeEvent = xcb::cast_event(&e);
              println!("Received Expose Event, {}", expose.window());
            },
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
