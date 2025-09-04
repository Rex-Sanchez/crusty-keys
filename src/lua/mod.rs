use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use i3ipc::I3Connection;
use mlua::{Function, Lua, Table};

use crate::{
    AppArgs, KeyMap,
    config::Config,
    error::AppResult,
    key_maps::{KeyMapOptions, KeyMaps, Map},
};

type I3 = Arc<Option<RwLock<I3Connection>>>;

pub(crate) struct LuaEngine {
    lua: mlua::Lua,
    i3: I3,
    pub(crate) keymaps: KeyMaps,
    config: Config,
}

impl LuaEngine {
    pub fn new(args: &AppArgs) -> AppResult<Self> {
        Self {
            lua: mlua::Lua::new(),
            i3: Arc::new(I3Connection::connect().ok().map(RwLock::new)),
            keymaps: KeyMaps::default(),
            config: Config::try_from(args).unwrap_or(Config::new()?),
        }
        .load()
    }

    fn load(mut self) -> AppResult<Self> {
        self.set_globals()?;

        let config_dir = &self.config.dir;

        // We need to load the package path recursivly so that we are able to use require in the
        // main config file
        self.lua
            .load(format!(
                r#"package.path = package.path .. ';{config_dir}/?.lua;{config_dir}/?/?.lua'"#,
            ))
            .exec()?;

        // Loading the main config file into the engine.
        self.lua.load(&self.config.cfg).exec()?;
        Ok(self)
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
                    cmd::spawn_in_terminal(args, opt);
                } else if let Some(cmd) = args.first() {
                    cmd::spawn(cmd, args[1..].to_vec(), opt);
                }
                Ok(())
            },
        )?;

        Ok(f)
    }

    fn i3_msg_func(&self) -> AppResult<Function> {
        let i3 = self.i3.clone();
        let f = self.lua.create_function(move |_lua: &Lua, args: String| {
            if let Some(i3_lock) = i3.as_ref()
                && let Ok(mut i3) = i3_lock.write()
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
                if let (Ok(map), Ok(mut maps)) = (Map::try_from(&keymap), keymaps.write()) {
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
struct RunOptions {
    env: HashMap<String, String>,
    in_terminal: bool,
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


mod cmd {
    use crate::{logger::log, lua::RunOptions};
    use std::{
        os::unix::process::CommandExt,
        process::{Command, Stdio},
    };

    pub fn spawn_in_terminal(args: Vec<&str>, opt: RunOptions) {
        if let Some((_, term)) =
            std::env::vars().find(|(k, v)| k.as_str() == "TERM" && !v.is_empty())
        {
            let args = ["-e"].into_iter().chain(args).collect();
            spawn(&term, args, opt);
        }
    }

    pub fn spawn(cmd: &str, args: Vec<&str>, opt: RunOptions) {
        let mut e = Command::new(cmd);
        e.args(&args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .process_group(0)
            .envs(opt.env);

        if let Err(e) = e.spawn() {
            log(e.to_string().as_str());
        }
    }
}
