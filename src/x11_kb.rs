use std::{collections::HashMap, ffi::c_ulong, fmt::Display};

use x11_dl::xlib::{self, AnyKey, AnyModifier, GrabModeAsync, True};

use crate::{logger::log, KeyMap};

type ListenerID = (i32, u32);

enum UnGrabKeyError {
    BadAccess,
    BadValue,
    BadWindow,
    Unknown(i32),
}

impl From<i32> for UnGrabKeyError {
    fn from(value: i32) -> Self {
        match value {
            1 => UnGrabKeyError::BadAccess,
            2 => UnGrabKeyError::BadValue,
            3 => UnGrabKeyError::BadWindow,
            _ => UnGrabKeyError::Unknown(value),
        }
    }
}

impl<'a> From<&'a UnGrabKeyError> for String {
    fn from(val: &'a UnGrabKeyError) -> Self {
        val.to_string()
    }
}

impl Display for UnGrabKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnGrabKeyError::BadAccess => f.write_str("Ungrabing Keys: BadAccess"),
            UnGrabKeyError::BadValue => f.write_str("Ungrabbing Keys: BadValue"),
            UnGrabKeyError::Unknown(e) => {
                f.write_fmt(format_args!("Ungrabbing Keys: Unknown error: {e}"))
            }
            UnGrabKeyError::BadWindow => f.write_str("Ungrabbing Keys: BadWindow"),
        }
    }
}

enum GrabKeyError<'a> {
    #[allow(unused)]
    AlreadyGrabbed(&'a KeyMap),
    GrabInvalidTime,
    GrabNotViewable,
    GrabFrozen,
    Unknown(i32),
}

impl<'a> From<i32> for GrabKeyError<'a> {
    fn from(value: i32) -> Self {
        match value {
            2 => GrabKeyError::GrabInvalidTime,
            3 => GrabKeyError::GrabNotViewable,
            4 => GrabKeyError::GrabFrozen,
            _ => GrabKeyError::Unknown(value),
        }
    }
}

impl<'a> From<&'a GrabKeyError<'a>> for String {
    fn from(val: &'a GrabKeyError<'a>) -> Self {
        val.to_string()
    }
}
impl<'a> Display for GrabKeyError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrabKeyError::Unknown(e) =>  f.write_fmt(format_args!("Grabbing Keys: Unknown error: {e}")),
            GrabKeyError::AlreadyGrabbed(keymap) => {
                f.write_fmt(format_args!("Grabbing Keys: Already Grabbed: {}", keymap.s))
            }
            GrabKeyError::GrabInvalidTime => f.write_str("Grabbing Keys: Invalid time"),
            GrabKeyError::GrabNotViewable => f.write_str("Grabbing Keys: Not viewable"),
            GrabKeyError::GrabFrozen => f.write_str("Grabbing Keys: Frozen"),
        }
    }
}

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

        // Because Numlock & Capslock are modifiers as well we need to add the keymaps with
        // these as well else the keymaps will not work if capslock and or numlock is on.
        let modifiers = keymap.map.modifiers.as_universal();
        

        unsafe {
            let keycode = (self.xlib.XKeysymToKeycode)(self.display, key as u64) as i32;
            modifiers
                .into_iter()
                .filter_map(|m| {
                    let result = (self.xlib.XGrabKey)(
                        self.display,
                        keycode,
                        m,
                        self.root,
                        True,
                        GrabModeAsync,
                        GrabModeAsync,
                    );

                    match result {
                        1 => Some((keycode, m)),
                        _ => {
                            let msg = GrabKeyError::from(result);
                            log(&msg);
                            eprintln!("{msg}");
                            None
                        }
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

    pub fn unregister_all(&self) {
        unsafe {
            let result = (self.xlib.XUngrabKey)(self.display, AnyKey, AnyModifier, self.root);
            if result != 0 {
                let msg = UnGrabKeyError::from(result);
                log(&msg);
                eprintln!("{msg}");
            }
        }
    }
    pub fn listen(&self) {
        unsafe {
            loop {
                let mut event: xlib::XEvent = std::mem::zeroed();
                (self.xlib.XNextEvent)(self.display, &mut event);

                if event.get_type() == xlib::KeyPress
                    && let Some(cb) = self
                        .handlers
                        .get(&(event.key.keycode as i32, event.key.state))
                {
                    let _ = cb.call::<()>(());
                }
            }
        }
    }
}
