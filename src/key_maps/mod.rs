pub mod kbcode;

use crate::key_maps::kbcode::{KbCode, KeyKind};

pub struct Map {
    pub modifiers: u32,
    pub code: KbCode,
}

impl TryFrom<&String> for Map {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut event = value
            .split("+")
            .fold(MapBuilder::default(), |mut event, s| {
                let s = s.trim();
                match KeyKind::from(s) {
                    KeyKind::Mod(kb_modifier) => event.modifiers |= kb_modifier.to_code(),
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
    pub modifiers: u32,
    pub code: Option<KbCode>,
}

impl MapBuilder {
    pub fn build(&mut self) -> Result<Map, ()> {
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
