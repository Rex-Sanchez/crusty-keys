require("util")

local HOME = "/home/n3m0"

kbd.keymap.set("<super>+d",
  function()
    kbd.util.run("rofi -show drun -show-icons")
  end, options("Rofi drun", "Rofi"))

kbd.keymap.set("<super>+<shift>+d",
  function()
    kbd.util.run("rofi -show run")
  end, options("Rofi Dmenu", "Rofi"))

kbd.keymap.set("<super>+1",
  function()
    kbd.util.run("rofi -show calc")
  end, options("Rofi Calc", "Rofi"))

kbd.keymap.set("<super>+<tab>",
  function()
    kbd.util.run("rofi -show")
  end, options("Rofi Tab window select", "Rofi"))

kbd.keymap.set("<super>+6",
  function()
    kbd.util.run_with_env("GDK_SCALE=1", "vesktop")
  end, options("Start Vesktop (Discord)", "Application"))

kbd.keymap.set("<super>+0",
  function()
    kbd.util.run(HOME .. "/.config/scripts/rofi-audio.sh")
  end, options("Select Audio Source","Rofi"))

kbd.keymap.set("<super>+<ctrl>+l",
  function()
    kbd.util.run("i3lock -c 31343C")
  end, options("I3lock - lock session", "I3"))

kbd.keymap.set("<super>+<enter>",
  function()
    kbd.util.run("alacritty")
  end, options("Open Alacritty", "Application"))

kbd.keymap.set("<super>+p",
  function()
    kbd.util.run("thunar")
  end, options("Open Thunar","Application"));

kbd.keymap.set("<super>+o",
  function()
    kbd.util.run_in_terminal(string.format("nvim %s/.config/i3/config",HOME))
  end, options("Open i3 config","Config"))

kbd.keymap.set("<super>+<shift>+<ctrl>+k",
  function()
    kbd.util.run_in_terminal("xkill")
  end, options("Kill window on click","Util"))


