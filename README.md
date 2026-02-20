# `sound_themer` — A theme-selected sound playing program
A Linux-focused program which plays sounds from a selected theme, using only the base name (e.g: `bell` --> `/usr/share/sounds/freedesktop/stereo/bell.oga`).

Makes uses of a `config.toml` file at `.config/sound_themer/config.toml` to change the theme settings.

Intended for system notifications so that theming is simple and logical.

---
<br/>

<div align="center">
  
[sound_themer](https://aur.archlinux.org/packages/sound_themer) is available on the Arch User Repository.

It can be installed via `paru -S sound_themer`, or any other AUR package installation service.

[![AUR version](https://img.shields.io/aur/version/sound_themer)](https://aur.archlinux.org/packages/sound_themer)
[![AUR votes](https://img.shields.io/aur/votes/sound_themer)](https://aur.archlinux.org/packages/sound_themer)
</div>


<br/>


## Usage
Play sound from config file's theme
```
sound_themer play message
sound_themer p message
```

List sounds in theme
```
sound_themer list
sound_themer ls
sound_themer l
```

Theme Overriding
```
sound_themer -t test_theme play message
sound_themer --theme test_theme play message
```

Extension Overriding
```
sound_themer -e wav play message
sound_themer --ext mp3 play message
```

Use `sound_themer help` to get more info about usage


<br/>


## Config
By default the config file is located at `.config/sound_themer/config.toml`, when the program is first run, if this file doesn't exist, the default config will be copied from `/etc/sound_themer/config.toml`

Example Config
``` toml
# Name of the selected sound theme
theme_name = "freedesktop"

[[themes]]
# Name of the sound theme folder
name = "freedesktop"

# Extension on the sound files
sound_ext = "oga"

# Directories where the sounds are found
directories = ["stereo"]

# Provide a mapping between certain phrases and their respective sound file name
mapping = {
  audio-change = "audio-volume-change",
  login = "service-login",
  logout = "service-logout",
  message = "message",
  power-plug = "power-plug",
  power-unplug = "power-unplug",
  dialog-info = "dialog-information",
  dialog-warning = "dialog-warning",
  dialog-error = "dialog-error",
  screen-capture = "screen-capture",
  device-added = "device-added",
  device-removed = "device-removed",
  camera-shutter = "camera-shutter",
  trash-empty = "trash-empty",
  complete = "complete"
}
```


<br/><br/>


## Requirements

* `wireplumber` (Pipewire) for playing sounds via `pw-play`
* `sound-theme-freedesktop` for a default theme
