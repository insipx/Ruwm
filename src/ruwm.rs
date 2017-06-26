extern crate xcb;
 pub struct Ruwm {
  connection: xcb::Connection,
  screen_num: i32,
}

impl Ruwm {
  pub fn new(dpy: &str) -> Option<Self> {
    println!("DPY: {}", dpy);
    let (connection, screen_num) = xcb::Connection::connect(Some(dpy)).unwrap();

   println!("Screen Num: {}", screen_num);

    Some( Ruwm{
      connection,
      screen_num,
    })
  }

  pub fn run(&self) {
    let setup = self.connection.get_setup();
    // get the screen whose number is screen_num

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

    let screen = setup.roots().nth(self.screen_num as usize).unwrap();
    let root = screen.root();
    println!("ROOT: {:?}", root);

    let window = self.connection.generate_id();
    let events : u16 = 
      (xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE) as u16;

    let values = [
      (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb:: EVENT_MASK_KEY_PRESS), 
    ];
    xcb::grab_key(&self.connection, true, root, xcb::MOD_MASK_2 as u16, 
      xcb::NO_SYMBOL, xcb::GRAB_MODE_ASYNC, xcb::GRAB_MODE_ASYNC);
    
    xcb::grab_button(self.connection, 0, root, events, xcb::GRAB_MODE_ASYNC, 
      xcb::GRAB_MODE_ASYNC, root, xcb::NONE, 1, xcb::MODE_MASK_1);
  
    xcb::grab_button(self.connection, 0, root, events, xcb::GRAB_MODE_ASYNC,
      xcb::GRAB_MODE_ASYNC, root, xcb::NONE, 3, xcb::MODE_MASK_1);    

    self.connection.flush();

    'event_loop : loop {
      let event = self.connection.wait_for_event();
      match event {
        Some(e) => {
          let r = e.response_type();
          if r == xcb::KEY_PRESS as u8 {
            let key_press : &xcb::KeyPressEvent = xcb::cast_event(&e);
            println!("Key '{}' pressed", key_press.detail());
            if key_press.detail() == 0x18 { //Q
              break 'event_loop;
            }
          }
          println!("Received some event: {}", e.response_type());
        },
        None => { }
      }
    }
  }
}
