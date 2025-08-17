use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use mlua::{Function, Lua, Table};

use crate::{
    key_maps::Map, lua::{KeyMapOptions, RunOptions, I3}, logger::log, KeyMap, Keymaps
};

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

    if let Err(e) = e.spawn(){
        log(e.to_string().as_str());
        eprintln!("spawn command failed: {e}");
    }
}

pub fn run(lua: &Lua) -> Result<Function, mlua::Error> {
    let f = lua.create_function(
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

pub fn i3_msg(lua: &Lua, i3: I3) -> Result<Function, mlua::Error> {
    let f = lua.create_function(move |_lua: &Lua, args: String| {
        if let Some(mutex) = i3.as_ref()
            && let Ok(mut i3) = mutex.write()
        {
            let _ = i3.run_command(&args);
        }
        Ok(())
    })?;
    Ok(f)
}

pub fn keymap_set(lua: &Lua, keymaps: Keymaps) -> crate::error::Result<Function> {
    let f = lua.create_function(
        move |_lua: &Lua, (keymap, cb, desc): (String, Function, Option<Table>)| {
            if let Ok(map) = Map::try_from(&keymap)
                && let Ok(mut maps) = keymaps.write()
            {
                maps.push(KeyMap {
                    map,
                    cb,
                    s: keymap,
                    options: { desc.as_ref().map(KeyMapOptions::from).unwrap_or_default() },
                });
            }
            Ok(())
        },
    )?;
    Ok(f)
}
