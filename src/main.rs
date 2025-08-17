mod error;
mod key_maps;
mod logger;
mod lua;
mod x11_kb;

use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use crate::{
    key_maps::Map,
    logger::log,
    lua::{KeyMapOptions, LuaEngine},
    x11_kb::X11Kb,
};
use clap::{Parser, Subcommand};
use mlua::Function;

#[derive(Subcommand)]
enum Mode {
    /// List Keybinds
    List,
    /// Run as Daemon
    Daemon,
}

#[derive(Parser)]
#[command(version)]
struct AppArgs {
    /// Mode to run as
    #[command(subcommand)]
    mode: Mode,
}

pub struct KeyMap {
    options: KeyMapOptions,
    s: String,
    map: Map,
    cb: Function,
}

#[derive(Default, Clone)]
pub struct Keymaps(pub Arc<RwLock<Vec<KeyMap>>>);

impl Deref for Keymaps {
    type Target = Arc<RwLock<Vec<KeyMap>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Keymaps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Keymaps {
    fn print_maps(&self) {
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

fn run() -> crate::error::Result<()> {
    let engine = LuaEngine::new()?;
    let keymaps = engine.keymaps.read().expect("Error: Could not keymap lock");
    let args = AppArgs::parse();

    match args.mode {
        Mode::List => engine.keymaps.print_maps(),
        Mode::Daemon => {
            let mut kb = X11Kb::new()?;
            // we should first unregister all keybindings before applying new once else binding will fail
            kb.unregister_all();
            kb.register(&keymaps);
            kb.listen();
        }
    }
    Ok(())
}

fn main() -> error::Result<()> {
    if let Err(e) = run() {
        log(e.to_string().as_str());
        eprintln!("{e}");
    }
    Ok(())
}
