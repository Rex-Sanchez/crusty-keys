mod error;
mod key_maps;
mod logger;
mod lua;
pub mod macros;
mod x11_kb;


use std::path::PathBuf;

use crate::{
    error::{AppError, AppResult},
    key_maps::KeyMap,
    logger::log,
    lua::LuaEngine,
    x11_kb::X11Kb,
};
use clap::{Parser, Subcommand};

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
    #[arg(long, short)]
    pub config: Option<PathBuf>,
    /// Mode to run as
    #[command(subcommand)]
    mode: Mode,
}

fn run() -> AppResult<()> {
    let args = AppArgs::parse();
    let engine = LuaEngine::new(&args)?;

    let keymaps = engine.keymaps.read().map_err(|_| AppError::ReadLockError)?;

    match args.mode {
        Mode::List => engine.keymaps.print_maps(),
        Mode::Daemon => {
            let mut kb = X11Kb::new()?;
            kb.register(&keymaps);
            kb.listen();
        }
    }
    Ok(())
}

fn main() -> AppResult<()> {
    if let Err(e) = run() {
        log(e.to_string().as_str());
        eprintln!("{e}");
    }
    Ok(())
}
