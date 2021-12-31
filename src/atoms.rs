use x11::xlib;

use crate::utils;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Atoms {
    pub MANAGER: xlib::Atom,
    pub NET_SYSTEM_TRAY_MESSAGE_DATA: xlib::Atom,
    pub NET_SYSTEM_TRAY_OPCODE: xlib::Atom,
    pub NET_SYSTEM_TRAY_ORIENTATION: xlib::Atom,
    pub NET_SYSTEM_TRAY_VISUAL: xlib::Atom,
    pub NET_WM_NAME: xlib::Atom,
    pub NET_WM_PID: xlib::Atom,
    pub NET_WM_PING: xlib::Atom,
    pub NET_WM_STATE: xlib::Atom,
    pub NET_WM_STATE_STICKY: xlib::Atom,
    pub NET_WM_SYNC_REQUEST: xlib::Atom,
    pub NET_WM_WINDOW_TYPE: xlib::Atom,
    pub NET_WM_WINDOW_TYPE_DIALOG: xlib::Atom,
    pub WM_DELETE_WINDOW: xlib::Atom,
    pub WM_PROTOCOLS: xlib::Atom,
    pub XEMBED: xlib::Atom,
    pub XEMBED_INFO: xlib::Atom,
}

impl Atoms {
    pub fn new(display: *mut xlib::Display) -> Self {
        unsafe {
            Self {
                MANAGER: utils::new_atom(display, "MANAGER\0"),
                NET_SYSTEM_TRAY_MESSAGE_DATA: utils::new_atom(
                    display,
                    "_NET_SYSTEM_TRAY_MESSAGE_DATA\0",
                ),
                NET_SYSTEM_TRAY_OPCODE: utils::new_atom(display, "_NET_SYSTEM_TRAY_OPCODE\0"),
                NET_SYSTEM_TRAY_ORIENTATION: utils::new_atom(
                    display,
                    "_NET_SYSTEM_TRAY_ORIENTATION\0",
                ),
                NET_SYSTEM_TRAY_VISUAL: utils::new_atom(display, "_NET_SYSTEM_TRAY_VISUAL\0"),
                NET_WM_NAME: utils::new_atom(display, "_NET_WM_NAME\0"),
                NET_WM_PID: utils::new_atom(display, "_NET_WM_PID\0"),
                NET_WM_STATE_STICKY: utils::new_atom(display, "_NET_WM_STATE_STICKY\0"),
                NET_WM_PING: utils::new_atom(display, "_NET_WM_PING\0"),
                NET_WM_STATE: utils::new_atom(display, "_NET_WM_STATE\0"),
                NET_WM_SYNC_REQUEST: utils::new_atom(display, "_NET_WM_SYNC_REQUEST\0"),
                NET_WM_WINDOW_TYPE: utils::new_atom(display, "_NET_WM_WINDOW_TYPE\0"),
                NET_WM_WINDOW_TYPE_DIALOG: utils::new_atom(display, "_NET_WM_WINDOW_TYPE_DIALOG\0"),
                WM_DELETE_WINDOW: utils::new_atom(display, "WM_DELETE_WINDOW\0"),
                WM_PROTOCOLS: utils::new_atom(display, "WM_PROTOCOLS\0"),
                XEMBED: utils::new_atom(display, "_XEMBED\0"),
                XEMBED_INFO: utils::new_atom(display, "_XEMBED_INFO\0"),
            }
        }
    }
}
