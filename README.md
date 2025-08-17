# Crusty-keys
Crusty-keys is a X11 shortcut daemon match like sxhkd but written in rust, and uses lua to configure itself it also has a buildin function for i3wm that uses the internal IPC so its more efficent then i3-msg


## Config
The config can be found in ~/.config/crusty-keys/config.lua
you can break up your config into multiple files and just require them in the main config.lua


## Lua functions and options
```lua


kbd.keymap.set("<keymap>", function() 
    -- Callback
end, 
{ -- all options are optional
    group = "Keymap group (optional)",
    desc = "Description of the keymap (optional)"
})


kbd.util.run("cmd", 
{ -- all options are optional 
    env = {
        "Somekey" = "some valiable", -- set a env variable before running the command.
    },
    in_terminal = false -- run this command in a terminal window default is false 
})

kbd.util.i3("i3 command") -- this is eq to cmd with i3-msg but it uses IPC so its a bit more efficient.


```

## Example

```lua
kbd.keymap.set("<super>+<shift>+<left>", function() 
    kbd.util.i3("move left") 
end, 
{ -- all options are optional
    group = "I3",
    desc = "Move container to the left"
})

kbd.keymap.set("<super>+<left>", function() 
    kbd.util.i3("focus left") 
end, 
{ -- all options are optional
    group = "I3",
    desc = "Move focus left"
})

kbd.keymap.set("<super>+1", function() 
    kbd.util.run("firefox") 
end, 
{ -- all options are optional
    group = "Applications",
    desc = "Run Firefox",
})

kbd.keymap.set("<super>+o", function() 
    kbd.util.run("nvim ~/.config/crusty-keys/config.lua", { in_terminal = true }) 
end, 
{ -- all options are optional
    group = "Configs",
    desc = "Open Crusty-keys config",
})

kbd.keymap.set("<super>+2", function() 
    kbd.util.i3("workspace 1") 
    kbd.util.run("vesktop", { env = { KDG_SCALE = 1 }}) 
end, 
{ -- all options are optional
    group = "Applications",
    desc = "Move to workspace 1 and open vesktop",
})


```


## Supported keymaps (for now there is no support for direct keycodes)
All keys are casted to lowercase so casing does not matter 

```
<super>
<ctrl>
<alt>
<shift>

<enter>
<space>
<backspace>
<esc>

<end>
<home>
<insert>
<del>

<up>
<down>
<right>
<left>

<pgdown>
<pgup>

<tab>

<equals>
<minus>

<quote>
<backtick>
<backslash>
<slash>
<semicolon>

<comma>
<period>

<f1>
<f2>
<f3>
<f4>
<f5>
<f6>
<f7>
<f8>
<f9>
<f10>
<f11>
<f12>
   
Any lowercase alpha numeric 

```
