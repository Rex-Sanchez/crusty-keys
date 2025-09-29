pub mod kbcode;

use std::{cmp::Ordering, ops::BitOrAssign, sync::{Arc, RwLock}};

use mlua::{Function, Table};
use x11_dl::xlib::{LockMask, Mod2Mask};

use crate::key_maps::kbcode::{KbCode, KbModifierCode, KbSym, KeyKind};

#[derive(Debug, Clone, Default)]
pub struct KeyMapOptions {
    pub group: Option<String>,
    pub desc: Option<String>,
}

impl From<Table> for KeyMapOptions {
    fn from(value: Table) -> Self {
        KeyMapOptions {
            group: value.get("group").ok(),
            desc: value.get("desc").ok(),
        }
    }
}

pub struct KeyMap {
    pub options: KeyMapOptions,
    pub s: String,
    pub map: Map,
    pub cb: Function,
}

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

pub enum KbCodeType{
    Sym(KbSym),
    Code(KbCode)
}


pub struct Map {
    pub modifiers: Modifier,
    pub code: KbCodeType,
}

impl TryFrom<&String> for Map {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let event = value
            .split("+")
            .fold(MapBuilder::default(), |mut map, s| {
                let s = s.trim();
                match KeyKind::from(s) {
                    KeyKind::Mod(kb_modifier) => map.modifiers |= kb_modifier,
                    KeyKind::KeySym(kb_code) => map.code = Some(KbCodeType::Sym(kb_code)),
                    KeyKind::KeyCode(kb_code) => map.code =  Some(KbCodeType::Code(kb_code)),
                    KeyKind::Unknown => map.code = None,
                }
                map
            });
        event.build()
    }
}

#[derive(Default)]
pub struct MapBuilder {
    pub modifiers: Modifier,
    pub code: Option<KbCodeType>,
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

#[derive(Default, Clone)]
pub struct KeyMaps(pub Arc<RwLock<Vec<KeyMap>>>);
crate::deref!(KeyMaps => Arc<RwLock<Vec<KeyMap>>>);

impl KeyMaps {
    pub fn print_maps(&self) {
        if let Ok(keymaps) = self.read() {
            
            let mut keymaps = keymaps
                .iter()
                .map(|m| (&m.s, &m.options))
                .collect::<Vec<(&String, &KeyMapOptions)>>();

            keymaps.sort_by(|(_, options_a), (_, options_b)| {
                if options_a.group.is_none() {
                    Ordering::Greater
                } else if options_b.group.is_none() {
                    Ordering::Less
                } else {
                    options_a.group.cmp(&options_b.group)
                }
            });

            println!(
                "| {:<40} | {:<50} | {:<40}",
                "Binding", "Description", "Groups"
            );

            let line = ["-"; 120].join("");
            println!("{line}");

            keymaps.into_iter().for_each(|(map, options)| {
                println!(
                    "| {:<40} | {:<50} | {:<40}",
                    map,
                    options.desc.as_ref().unwrap_or(&"".to_string()),
                    options.group.as_ref().unwrap_or(&"".to_string())
                );
                println!("{line}");
            });
        }
    }
}




