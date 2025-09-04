# Crusty-keys
Crusty-keys is a X11 shortcut daemon allot like sxhkd but written in rust, and uses lua to configure itself it also has a buildin function for i3wm that uses the internal IPC so its more efficent then i3-msg


## Config
The config can be found in ~/.config/crusty-keys/config.lua
you can break up your config into multiple files and just require them in the main config.lua


## Lua functions and options
```lua


ck.keymap.set("<keymap>", function() 
    -- Callback
end, 
{ -- all options are optional
    group = "Keymap group (optional)",
    desc = "Description of the keymap (optional)"
})


ck.util.run("cmd", 
{ -- all options are optional 
    env = {
        "key" = "val", -- set a env variable before running the command.
    },
    in_terminal = false -- run this command in a terminal window default is false 
})

ck.util.i3("i3 command") -- this is eq to cmd with i3-msg but it uses IPC so its a bit more efficient.


```

## Example

```lua
ck.keymap.set("<super>+<shift>+<left>", function() 
    ck.util.i3("move left") 
end, 
{ -- all options are optional
    group = "I3",
    desc = "Move container to the left"
})

ck.keymap.set("<super>+<left>", function() 
    ck.util.i3("focus left") 
end, 
{ -- all options are optional
    group = "I3",
    desc = "Move focus left"
})

ck.keymap.set("<super>+1", function() 
    ck.util.run("firefox") 
end, 
{ -- all options are optional
    group = "Applications",
    desc = "Run Firefox",
})

ck.keymap.set("<super>+o", function() 
    ck.util.run("nvim ~/.config/crusty-keys/config.lua", { in_terminal = true }) 
end, 
{ -- all options are optional
    group = "Configs",
    desc = "Open Crusty-keys config",
})



-- We have all powers of lua so we can easily loop through some things
local workspaces = {
    { ws = 1, key = "f1" }, 
    { ws = 2, key = "f2" }, 
    { ws = 3, key = "f3" }, 
    { ws = 4, key = "f4" }, 
}

for _, k in ipairs(workspaces) do 
    ck.keymap.set(string.format("<super>+<%s>",k.key), function() 
        ck.util.i3(string.format("workspace number ",k.ws)) 
    end, 
    { -- all options are optional
        group = "I3",
        desc = "Focus workspace number " .. k.ws
    })
    
    ck.keymap.set(string.format("<super>+<shift>+<%s>",k.key), function() 
        ck.util.i3(string.format("move container to workspace %s",k.ws)) 
    end, 
    { -- all options are optional
        group = "I3",
        desc = "Move container to workspace number " .. k.ws
    })

end


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

<f1>,<f2>,<f3>,<f4>,<f5>,<f6>,<f7>,<f8>,<f9>,<f10>,
<f11>,<f12>,<f13>,<f14>,<f15>,<f16>,<f17>,<f18>,<f19>,<f20>,
<f21>,<f22>,<f23>,<f24>,<f25>,<f26>,<f27>,<f28>,<f29>,<f30>,
<f31>,<f32>,<f33>,<f34>,<f35>

<KP0>
<KP1>
<KP2>
<KP3>
<KP4>
<KP5>
<KP6>
<KP7>
<KP8>
<KP9>


Any lowercase alpha numeric 

```
