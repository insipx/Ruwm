extern crate xcb;
use ruwm::scancode;
use ruwm::err::RuwmError;

use self::xcb::ffi::base::xcb_generic_event_t;

pub struct Handlers;
/*
 * Handlers for different X events.
 * Comments/Structure Shamelessly copied from i3, and modified to fit ruwm
 */
#[allow(dead_code)]
impl Handlers {
    /*
     * When a mouse button is pressed, this callback is called
     */
    pub fn handle_button_press(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        let button_press: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&event) };
        println!("Button '{:X}' pressed", button_press.detail());
        Ok(false)
    }

    /*
     * When a mouse button is released, this callback is called
     */

    pub fn handle_button_release(
        event: xcb::Event<xcb_generic_event_t>,
    ) -> Result<bool, RuwmError> {
        let button_release: &xcb::ButtonReleaseEvent = unsafe { xcb::cast_event(&event) };
        println!("Button {:X} release", button_release.detail());
        Ok(false)
    }

    /*
     * Called when a key is pressed
     * Create an enum from config file of Key Combinations and their default mappings
     * that way, can just import it and do scancode::Scancode::Q.run()
     */
    pub fn handle_key_press(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
        let code = scancode::Scancode::new(key_press.detail()).unwrap();
        println!(
            "Key '{}', Hex: '{:X}' pressed, scancode: {:?}",
            key_press.detail(),
            key_press.detail(),
            code
        );
        if code == scancode::Scancode::Q {
            return Ok(true);
        }
        return Ok(false);
    }

    /*
     * when a window changes sizes or is viewed on the screen
     * means we need to redraw our windows ( = title bar)
     * so let's handle it :->
     */
    pub fn handle_expose(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        let expose: &xcb::ExposeEvent = unsafe { xcb::cast_event(&event) };
        println!("Received Expose Event, {:X}", expose.window());
        Ok(false)
    }

    /*
     * When the user moves the mouse pointer onto a window, this event is triggered
     * and callback called
     */
    pub fn handle_enter_notify(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * When the user moves the mouse but does not change the active window
     * (e.g. when having no windows opened but moving mouse on the root screen
     * and crossing virtual screen boundaries), this callback gets called
     */
    pub fn handle_motion_notify(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * Called when keyboard mapping changes (for example, using Xmodmap),
     * we need to update our key bindings then (re-translate symbols)
     *
     */
    pub fn handle_mapping_notify(
        event: xcb::Event<xcb_generic_event_t>,
    ) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * A new window appeared on the screen (=was mapped), so let's manage it.
     *
     */
    pub fn handle_map_request(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        let map: &xcb::MapRequestEvent = unsafe { xcb::cast_event(&event) };
        println!("Map Request!: {:?} ", map.window());
        Ok(false)
    }

    /*
     * Configure requests are received when the application wants to resize
     * windows on their own
     *
     * We generate a synthetic configure notify event to signalize the client
     * its "new" position.
     */
    pub fn handle_configure_request(
        event: xcb::Event<xcb_generic_event_t>,
    ) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * Gets triggered upon a RandR screen change event, that is when the user
     * changes the screen configuraton in any way (mode, position, ... )
     *
     */
    pub fn handle_screen_change(event: xcb::Event<xcb_generic_event_t>) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * Our window decorations were unmapped. That means, the window will be killed
     * now, so we better clean up before (i3, not sure if this will be the case with ruwm)
     *
     */
    pub fn handle_unmap_notify_event(
        event: xcb::Event<xcb_generic_event_t>,
    ) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    /*
     * A destroy notify event is sent when the window is not unmapped, but
     * immediately destroyed (for example when starting a window and immediately
     * killing the program which started it).
     *
     * We just pass on the event to the unmap notify handler ( by copying the important fields
     * in the event data structure)
     *
     */
    pub fn handle_destroy_notify_event(
        event: xcb::Event<xcb_generic_event_t>,
    ) -> Result<bool, RuwmError> {
        unimplemented!();
    }

    // TODO lots of parameters for this one, get to it later
    // i3 as ref
    // fn handle_window_name_change(....)

    pub fn event_is_ignored() -> Result<bool, RuwmError> {
        unimplemented!();
    }

    pub fn check_crossing_screen_boundary(x: u32, y: u32) {
        unimplemented!();
    }
}
