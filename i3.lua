function options(desc)
  return { desc = desc, catagory = "I3" }
end

-- I3 control
kbd.keymap.set("<super>+c", function() kbd.util.i3("reload") end, options("Reload i3"))
kbd.keymap.set("<super>+r", function() kbd.util.i3("restart") end, options("Restart i3"))
kbd.keymap.set("<super>+e", function() kbd.util.i3("exit") end, options("Exit i3"))
kbd.keymap.set("<super>+<shift>+q", function() kbd.util.i3("kill") end, options("Kill focused application"))


-- Move through workspaces
local ws = {
  key = { "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>", "<F8>", "<F9>", "<F10>", "<F11>", "<F12>" },
  workspace = { "1", '2', '3', '4', '11', '12', '13', '14', '21', '22', '23', '24' }
}
for i = 1, #ws.key do
  kbd.keymap.set(string.format("<super>+%s", ws.key[i]),
    function()
      kbd.util.i3(string.format("workspace number %s", ws.workspace[i]))
    end, options(string.format("Move focus to workspace %s", ws.workspace[i])))

  kbd.keymap.set(string.format("<super>+<shift>+%s", ws.key[i]),
    function() kbd.util.i3(string.format("move container to workspace number %s", ws.workspace[i])) end,
    options(string.format("Move container to workspace %s", ws.workspace[i]))
  )

  kbd.keymap.set(string.format("<super>+<ctrl>+<shift>+%s", ws.key[i]),
    function()
      kbd.util.i3(string.format("move container to workspace number %s", ws.workspace[i]))
      kbd.util.i3(string.format("workspace number %s", ws.workspace[i]))
    end, options(string.format("Move and focus to workspace %s", ws.workspace[i])))
end



-- Resizing
local resize = {
  keys = { "left", "down", "up", "right" },
  cmd = { "grow height", "shrink height", "grow width", "shrink width" },
}
for i = 1, #resize.keys do
  kbd.keymap.set(string.format("<super>+<alt>+<%s>", resize.keys[i]),
    function()
      kbd.util.i3(string.format("resize %s 10 px or 10 ppt", resize.cmd[i]))
    end,
    options(string.format("Resize container: %s", resize.cmd[i]))
  )
end


-- Move and focus windows
local dir = { "left", "right", "down", "up" }
for i = 1, #dir do
  kbd.keymap.set(string.format("<super>+<%s>", dir[i]), function() kbd.util.i3(string.format("focus %s", dir[i])) end,
    options(string.format("Move focus %s", dir[i])))

  kbd.keymap.set(string.format("<super>+<shift>+<%s>", dir[i]),
    function() kbd.util.i3(string.format("move %s", dir[i])) end,
    options(string.format("Move container %s", dir[i])))
end


-- Splits
kbd.keymap.set("<super>+h", function() kbd.util.i3("split h") end, options("Split Horizantal"))
kbd.keymap.set("<super>+v", function() kbd.util.i3("split v") end, options("Split Vertical"))


-- Tiling mode
kbd.keymap.set("<super>+f", function() kbd.util.i3("fullscreen toggle") end, options("Toggle fullscreen, Mode"))
kbd.keymap.set("<super>+s", function() kbd.util.i3("layout stacked") end,options("Stack Layout Mode"))
kbd.keymap.set("<super>+w", function() kbd.util.i3("layout tabbed") end, options("Tabbed Layout Mode"))
kbd.keymap.set("<super>+e", function() kbd.util.i3("layout toggle split") end, options("Split Layout Mode"))
kbd.keymap.set("<super>+<shift>+<space>", function() kbd.util.i3("floating toggle") end,options("Toggle Floating Mode"))


-- Focus modes
kbd.keymap.set("<super>+<space>", function() kbd.util.i3("focus mode_toggle") end)
kbd.keymap.set("<super>+a", function() kbd.util.i3("focus parent") end)
kbd.keymap.set("<super>+z", function() kbd.util.i3("focus child") end)


-- Gaps
kbd.keymap.set("<super>+g", function() kbd.util.i3("gaps inner current toggle 20") end, options("Toggle gaps"))
