pub mod kbcode;

use std::ops::{BitOrAssign};

use x11_dl::xlib::{LockMask, Mod2Mask};

use crate::key_maps::kbcode::{KbCode, KbModifierCode, KeyKind};

#[derive(Default)]
pub struct Modifier(u32);

impl Modifier {
    pub fn as_universal(&self) -> [u32; 4] {
        [
            self.0,
            self.0 | Mod2Mask,
            self.0 | LockMask,
            self.0 | LockMask | Mod2Mask,
        ]
    }
}

crate::deref!(Modifier => u32);

impl From<u32> for Modifier {
    fn from(value: u32) -> Self {
        Modifier(value)
    }
}

impl BitOrAssign<KbModifierCode> for Modifier {
    fn bitor_assign(&mut self, rhs: KbModifierCode) {
        self.0 |= rhs.to_code()
    }
}

pub struct Map {
    pub modifiers: Modifier,
    pub code: KbCode,
}

impl TryFrom<&String> for Map {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let event = value
            .split("+")
            .fold(MapBuilder::default(), |mut event, s| {
                let s = s.trim();
                match KeyKind::from(s) {
                    KeyKind::Mod(kb_modifier) => event.modifiers |= kb_modifier,
                    KeyKind::Code(kb_code) => event.code = Some(kb_code),
                    KeyKind::Unknown => event.code = None,
                }
                event
            });
        event.build()
    }
}

#[derive(Default)]
pub struct MapBuilder {
    pub modifiers: Modifier,
    pub code: Option<KbCode>,
}

impl MapBuilder {
    pub fn build(mut self) -> Result<Map, ()> {
        self.code.take().map_or_else(
            || Err(()),
            |code| {
                Ok(Map {
                    modifiers: self.modifiers,
                    code,
                })
            },
        )
    }
}
