#![allow(non_upper_case_globals)]
use std::{collections::HashMap, ffi::c_ulong};

use x11_dl::xlib::{self, BadAccess, BadValue, BadWindow, GrabModeAsync, True};

use crate::{KeyMap, logger::log};

type ListenerID = (i32, u32);

pub struct X11Kb<'a> {
    display: *mut xlib::Display,
    root: c_ulong,
    xlib: xlib::Xlib,
    handlers: HashMap<ListenerID, &'a mlua::Function>,
}

impl<'a> X11Kb<'a> {
    pub fn new() -> crate::error::AppResult<Self> {
        let xlib = xlib::Xlib::open()?;

        unsafe {
            let display = (xlib.XOpenDisplay)(std::ptr::null());
            let root = (xlib.XDefaultRootWindow)(display);
            let mut supported_rtrn = std::mem::zeroed();
            (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

            Ok(Self {
                display,
                root,
                xlib,
                handlers: HashMap::new(),
            })
        }
    }

    fn grab_key(&self, keymap: &'a KeyMap) -> Vec<(i32, u32)> {
        let key = keymap.map.code.to_code();

        unsafe {
            let keycode = (self.xlib.XKeysymToKeycode)(self.display, key as u64) as i32;

            // Because Numlock & Capslock are modifiers as well we need to add the keymaps with
            // these as well. Else the keymap will not work if capslock and or numlock is on.
            keymap
                .map
                .modifiers
                .as_universal()
                .into_iter()
                .filter_map(|modifier| {

                    // We first have to unregister our key grab before we can register it again
                    // this so that if any othere window has a grab on the keymap it is first
                    // undone, this is needed because when we register a keygrab when its still
                    // grabbed be a different window the grab will fail
                    match (self.xlib.XUngrabKey)(self.display, keycode, modifier, self.root) as u8 {
                        BadValue => {
                            log("Keymap unregister error: BadValue");
                        }
                        BadWindow => {
                            log("Keymap register error: BadAccess");
                        }
                        _ => (),
                    }

                    match (self.xlib.XGrabKey)(
                        self.display,
                        keycode,
                        modifier,
                        self.root,
                        True,
                        GrabModeAsync,
                        GrabModeAsync,
                    ) as u8
                    {
                        BadAccess => {
                            log("Keymap register error: BadAccess");
                            None
                        }
                        BadValue => {
                            log("keymap register error: BadValue");
                            None
                        }
                        BadWindow => {
                            log("kemap register error: BadWindow");
                            None
                        }
                        _ => Some((keycode, modifier)),
                    }
                })
                .collect::<Vec<_>>()
        }
    }

    pub fn register(&mut self, keymaps: &'a [KeyMap]) {
        keymaps.iter().for_each(|map| {
            self.grab_key(map).into_iter().for_each(|id| {
                self.handlers.insert(id, &map.cb);
            });
        });
    }
    pub fn listen(&self) {
        unsafe {
            loop {
                let mut event: xlib::XEvent = std::mem::zeroed();
                (self.xlib.XNextEvent)(self.display, &mut event);

                if let (true, Some(cb)) = (
                    event.get_type() == xlib::KeyPress,
                    self.handlers
                        .get(&(event.key.keycode as i32, event.key.state)),
                ) {
                    let _ = cb.call::<()>(());
                }
            }
        }
    }
}
