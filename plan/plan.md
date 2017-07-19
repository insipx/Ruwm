# Events
	- Mouse events
		- how will handle them?
		- 
	- Touch Events
		- drag to reorganize tiles

# Key Configuration
	- i3like configuration
	- Config file parser
	Future: 
		- put files in system dirs '/usr/share/????' && '/home/$USER/.config/ruwm/config'
	- Scancodes
		- create a thin wrapper around scancode::Scancode in order to be able to implement
		Hash, PartialEq, Eq, and Hash for it.
		- can translate scancodes into one-character-codes
		- TODO Look into XKB. This may be my savour here for different keyboard layouts
			- but i'm really not sure b/c i have no clue what it does

# Map Event
	- data structure (B-Tree)
	- on Map Request Event
		- feed window into B-Tree
		- reparent window
	- Geometry of Windows

# IPC 
	- IPC for communication with Ruwm
	- seperate crate/package/logic
	- communicate with eventual RuwmBar
	- UNIX Sockets

# EWMH Compliance (ICCCM Compliance?)
	- Gotta read through the rest of that spec x.X

# Triage
	- Map them windows
	- default keybinding map
	- start on config file
	- enum for keybindings?
	- i should check if my stream is working

## Learning
	- WTF is XKB and how do i use it
	- managing window properties
	- Fonts and X, htf do i use them
	- HiDPI fonts and X
	- X Touch Support

## Learned
	- WTF is an Atom
	- grab_key
	- Substructure Redirection

### Links:
https://specifications.freedesktop.org/wm-spec/wm-spec-1.3.html
https://www.x.org/wiki/guide/concepts/#index12h4
https://tronche.com/gui/x/icccm/

### Docs
https://xcb.freedesktop.org/XcbApi/
https://www.x.org/docs/XKB/XKBproto.pdf
https://www.x.org/wiki/XKB/

### Rust Binding Docs
http://rtbo.github.io/rust-xcb/xcb/
https://docs.rs/xkb/0.1.2/xkb/

### Libraries

#### In-Use
https://github.com/rtbo/rust-xcb

#### Plan To Use
https://crates.io/crates/xkb

#### Not Entirely sure how to use yet, but seem useful
https://crates.io/crates/xkbcommon-sys
https://crates.io/crates/xcb-util
https://crates.io/crates/xkbcommon