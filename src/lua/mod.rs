use std::{
    collections::HashMap,
    fs::read_to_string,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{Arc, RwLock},
};

use i3ipc::I3Connection;
use mlua::{Function, Lua, Table};

use crate::{
    error::{AppError, AppResult}, key_maps::{KeyMapOptions, KeyMaps, Map}, logger::log, KeyMap 
};

pub type I3 = Arc<Option<RwLock<I3Connection>>>;

pub struct LuaEngine {
    lua: mlua::Lua,
    i3: I3,
    pub keymaps: KeyMaps,
}

impl LuaEngine {
    pub fn new() -> Self {
        Self {
            lua: mlua::Lua::new(),
            i3: Arc::new(I3Connection::connect().ok().map(RwLock::new)),
            keymaps: KeyMaps::default(),
        }
    }

    pub fn load(&mut self) -> AppResult<()> {
        let (config_dir, config_file_path) = config_file_path()?;

        let config = read_to_string(&config_file_path)?;

        let _ = self.set_globals();

        // We need to load the package path recursivly so that we are able to use require in the
        // main config file
        self.lua
            .load(format!(
                r#"package.path = package.path .. ';{config_dir}/?.lua;{config_dir}/?/?.lua'"#,
            ))
            .exec()?;

        // Loading the main config file into the engine.
        self.lua.load(&config).exec()?;
        Ok(())
    }
    fn set_globals(&mut self) -> crate::error::AppResult<()> {
        // main table
        let ck = self.lua.create_table()?;

        // keymap
        ck.set("keymap", self.create_keymap_table()?)?;
        ck.set("util", self.create_util_table()?)?;

        let _ = self.lua.globals().set("ck", ck);
        Ok(())
    }

    fn create_keymap_table(&self) -> AppResult<Table> {
        let keymap = self.lua.create_table()?;
        let _ = keymap.set("set", self.keymap_set_func()?);
        Ok(keymap)
    }

    fn create_util_table(&self) -> AppResult<Table> {
        let util_table = self.lua.create_table()?;
        util_table.set("i3", self.i3_msg_func()?)?;
        util_table.set("run", self.run_func()?)?;
        Ok(util_table)
    }
}

// Global Lua functions impl
impl LuaEngine {
    fn run_func(&self) -> Result<Function, mlua::Error> {
        let f = self.lua.create_function(
            move |_lua: &Lua, (args, options): (String, Option<Table>)| {
                let opt = RunOptions::from(options);
                let args = args.split_whitespace().collect::<Vec<&str>>();

                if opt.in_terminal {
                    spawn_cmd_in_terminal(args, opt);
                } else if let Some(cmd) = args.first() {
                    spawn_cmd(cmd, args[1..].to_vec(), opt);
                }

                Ok(())
            },
        )?;

        Ok(f)
    }

    fn i3_msg_func(&self) -> AppResult<Function> {
        let i3 = self.i3.clone();
        let f = self.lua.create_function(move |_lua: &Lua, args: String| {
            if let Some(mutex) = i3.as_ref()
                && let Ok(mut i3) = mutex.write()
            {
                let _ = i3.run_command(&args);
            }
            Ok(())
        })?;
        Ok(f)
    }

    fn keymap_set_func(&self) -> AppResult<Function> {
        let keymaps = self.keymaps.clone();
        let f = self.lua.create_function(
            move |_lua: &Lua, (keymap, cb, desc): (String, Function, Option<Table>)| {
                if let Ok(map) = Map::try_from(&keymap)
                    && let Ok(mut maps) = keymaps.write()
                {
                    maps.push(KeyMap {
                        map,
                        cb,
                        s: keymap,
                        options: desc.map(KeyMapOptions::from).unwrap_or_default(),
                    });
                }
                Ok(())
            },
        )?;
        Ok(f)
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

pub fn spawn_cmd_in_terminal(args: Vec<&str>, opt: RunOptions) {
    if let Some((_, term)) = std::env::vars().find(|(k, v)| k.as_str() == "TERM" && !v.is_empty()) {
        let args = ["-e"].into_iter().chain(args).collect();
        spawn_cmd(&term, args, opt);
    }
}

pub fn spawn_cmd(cmd: &str, args: Vec<&str>, opt: RunOptions) {
    let mut e = Command::new(cmd);
    e.args(&args)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .process_group(0)
        .envs(opt.env);

    if let Err(e) = e.spawn() {
        log(e.to_string().as_str());
        eprintln!("spawn command failed: {e}");
    }
}

fn config_file_path() -> AppResult<(String, PathBuf)> {
    let home = std::env::home_dir().ok_or(AppError::HomeEnvNotSet)?;
    let config_dir_path = home.join(PathBuf::from(".config/crusty-keys"));
    let config_file_path = config_dir_path.join(PathBuf::from("config.lua"));

    // create a new config dir in $USER/.config
    // and generate a empty config.lua inside of it.
    if config_dir_path.canonicalize().ok().is_none() {
        let _ = std::fs::create_dir_all(&config_dir_path);
        let _ = std::fs::File::create_new(&config_file_path);
        println!("New config file created.")

    // if the $USER/.config exists but no config file is there we create a new one
    } else if config_file_path.canonicalize().ok().is_none() {
        let _ = std::fs::File::create_new(&config_file_path);
        println!("New config file created.")
    }

    let path_string = config_dir_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or(AppError::ConfigCouldNotBeCreated)?;

    Ok((path_string, config_file_path))
}
