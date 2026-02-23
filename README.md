# `sound_themer` — A theme-selected sound playing program
A Linux-focused program which plays sounds from a selected theme, using only the base name (e.g: `bell` --> `/usr/share/sounds/freedesktop/stereo/bell.oga`).
Intended for system notifications so that theming is simple and logical.

Makes uses of a `config.toml` file at `.config/sound_themer/config.toml` to change the theme settings.
The themes can have multiple folders inside which contain the sound files, but each inner directory must be set in the `config.toml` file.

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
### Play sound from config file's theme
```
sound_themer play <SOUND_NAME>
sound_themer p <SOUND_NAME>
```

### List sounds in theme
```
sound_themer list
sound_themer ls
sound_themer l
```

### Theme Overriding
```
sound_themer -t test_theme <COMMAND>
sound_themer --theme test_theme <COMMAND>
```

### Duration Overriding
```
sound_themer play -d 1.2 <SOUND_NAME>
sound_themer play -d 1.2s <SOUND_NAME>
sound_themer play -d 1200ms <SOUND_NAME>
sound_themer play -d 10.2% <SOUND_NAME>
sound_themer play --duration 1.2 <SOUND_NAME>
```

### Randomised Theme
```
sound_themer -r <COMMAND>
sound_themer --random <COMMAND>
```

### More Information
Use `sound_themer help` to get more info about usage

### Filename Mapping
Allows certain values to be overwritten by a configured value, increasing the ease-of-use across multiple themes.

`sound_themer play audio-change` --> `sound_themer play audio-volume-change`

If no mapping is set for a theme, then the default mapping will be the mapping for `freedesktop`.
To remove the mapping for a theme, use `mapping = {}` in the `config.toml`.

Detailed mapping entries can be defined, which set not only the name, but the duration for a specified keyword in the theme.
The format for the duration is identical to the `--duration` flag.

`login = {name = "service-login", duration = "50%"}`


<br/>


## Config
By default the config file is located at `.config/sound_themer/config.toml`, when the program is first run, if this file doesn't exist, the default config will be copied from `/etc/sound_themer/config.toml`

### Example Config
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
  login = {name = "service-login", duration = "100%"}, # Detailed mappings can be provided where the duration is also set
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
