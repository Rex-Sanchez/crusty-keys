use std::{
    collections::HashMap,
    fs::read_to_string,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use i3ipc::I3Connection;
use mlua::{Lua, Table};

use crate::{
    Keymaps,
    error::{AppError, Result},
    lua::functions::{i3_msg, keymap_set, run},
};

mod functions;

fn config_file_path() -> Option<(PathBuf, PathBuf)> {
    let home = std::env::home_dir()?;
    let config_dir = PathBuf::from(".config/crusty-keys");
    let config_path = home.join(config_dir);
    let config_file_path = config_path.join(PathBuf::from("config.lua"));

    if config_path.canonicalize().ok().is_none() {
        let _ = std::fs::create_dir_all(&config_path);
        let _ = std::fs::File::create_new(&config_file_path);
        println!("New config file created.")
    } else if config_file_path.canonicalize().ok().is_none() {
        let _ = std::fs::File::create_new(&config_file_path);
        println!("New config file created.")
    }

    Some((config_path, config_file_path))
}

pub type I3 = Arc<Option<RwLock<I3Connection>>>;

pub struct LuaEngine {
    lua: mlua::Lua,
    i3: I3,
    pub keymaps: Keymaps,
}

impl LuaEngine {
    pub fn new() -> Result<Self> {
        let mut s = Self {
            lua: mlua::Lua::new(),
            i3: Arc::new(I3Connection::connect().ok().map(RwLock::new)),
            keymaps: Keymaps::default(),
        };

        let _ = s.set_globals();

        let (config_dir, config_file_path) =
            config_file_path().ok_or(AppError::ConfigCouldNotBeCreated)?;

        let config_dir = config_dir
            .to_str()
            .ok_or(AppError::ConfigCouldNotBeCreated)?;

        let config = read_to_string(&config_file_path)?;

        s.lua
            .load(format!(
                r#"package.path = package.path .. ';{config_dir}/?.lua;{config_dir}/?/?.lua'"#,
            ))
            .exec()?;

        s.lua.load(&config).exec()?;

        Ok(s)
    }

    pub fn set_globals(&mut self) -> crate::error::Result<()> {
        // main table | rmux.
        let kbd = self.lua.create_table()?;

        // keymap | kbd.keymap
        kbd.set(
            "keymap",
            create_keymap_table(&self.lua, self.keymaps.clone())?,
        )?;
        kbd.set("util", create_util_table(&self.lua, self.i3.clone())?)?;

        let _ = self.lua.globals().set("kbd", kbd);
        Ok(())
    }
}

fn create_keymap_table(lua: &Lua, keymaps: Keymaps) -> crate::error::Result<Table> {
    let keymap = lua.create_table()?;
    let _ = keymap.set("set", keymap_set(lua, keymaps)?);
    Ok(keymap)
}

fn create_util_table(lua: &Lua, i3: I3) -> crate::error::Result<Table> {
    let util_table = lua.create_table()?;
    util_table.set("i3", i3_msg(lua, i3)?)?;
    util_table.set("run", run(lua)?)?;

    Ok(util_table)
}

#[derive(Debug, Clone, Default)]
pub struct KeyMapOptions {
    pub group: Option<String>,
    pub desc: Option<String>,
}

impl From<&Table> for KeyMapOptions {
    fn from(value: &Table) -> Self {
        let group = value.get("group").ok();
        let desc = value.get("desc").ok();
        KeyMapOptions { group, desc }
    }
}

#[derive(Default, Debug)]
pub struct RunOptions {
    pub env: HashMap<String, String>,
    pub in_terminal: bool,
}

impl From<Option<Table>> for RunOptions {
    fn from(mut value: Option<Table>) -> Self {
        value.take().map_or_else(Self::default, |table| Self {
            env: table
                .get::<HashMap<String, String>>("env")
                .unwrap_or_default(),
            in_terminal: table.get::<bool>("in_terminal").ok().unwrap_or_default(),
        })
    }
}
