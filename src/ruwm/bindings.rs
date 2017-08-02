use ruwm::xcb;
use ruwm::xkbcommon;
use ruwm::xkbcommon::xkb;
use std::ptr;
use self::xcb::ffi::base::xcb_generic_event_t;

use ruwm::err::RuwmError;
#[allow(dead_code)]
enum InputType {
    Keyboard,
    Mouse,
}

#[allow(dead_code)]
pub struct Binding {
    /*
     * The type of input this binding is for (InputType enum)
     */
    input_type: InputType,
    keycode: u32,
	/* 
	 * Symbol the user specified in configfile. This needs to be 
	 * stored with the binding to be able to re-convert it into a keycode
	 * if the keyboard mapping changes (using Xmodmap for example)
	 */
	// symbol: &str,

	/* Command, like in command mode */
	// command: &str,
}

#[allow(dead_code)]
impl Binding {
    pub fn new() -> Result<Binding, RuwmError> {
        unimplemented!();
    }
}

#[allow(dead_code)]
pub struct Bindings;

impl Bindings {
    #[allow(dead_code)]
    pub fn new(conn: xcb::base::Connection) -> Result<Bindings, RuwmError> {
        // xkb_x11_setup_xkb_extension(conn, )
        // get bindings
        // configure bindings
        // validate bindings
        // return valid bindings
        // For now, just return an empty struct
        let mut first_xkb_event: u8;
        let mut null: u16 = 0;
        xkb::x11::setup_xkb_extension(
            &conn,
            xkb::x11::MIN_MAJOR_XKB_VERSION,
            xkb::x11::MIN_MINOR_XKB_VERSION,
            xkb::x11::SetupXkbExtensionFlags::NoFlags,
            &mut null,
            &mut null,
            &mut first_xkb_event,
            &mut (null as u8),
        );
        Ok(Bindings {})
    }

    #[allow(dead_code)]
    pub fn get_binding_from_xcb_event(event: xcb::Event<xcb_generic_event_t>) {
        unimplemented!();
    }

    /*
     * Configures bindings with XCB
     *
     */
    pub fn configure_bindings() {
        unimplemented!();
    }

    /*
     * Check for duplicate bindings in binding
     * Datastructure
     *
     */
    pub fn check_for_duplicate_bindings() {
        unimplemented!();
    }

    /*
     * Validate bindings
     * Check for duplicates
     * And do anything else that needs to be done
     * in order to avoid errors
     *
     */
    pub fn validate_bindings() {
        unimplemented!();
    }
}
