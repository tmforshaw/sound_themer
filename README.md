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

<br/>

## Performance
Some of the functions have been benchmarked to ensure that the program will play sounds as fast as possible.
Globally available variables like `CONFIG` or `SELECTED_THEME` are forcibly reset for each benchmark iteration so that their lazy-evaluation can be benchmarked too.

### Play Sound For Duration of Zero
A sound duration of zero has been chosen for this benchmark; the code only takes ~150 microseconds to run, so any non-zero duration of sound will completely dominate the benchmark, obscuring the benchmark time for just the code.
```
Evaluate CLI: "sound_themer play --duration 0 complete"
                        time:   [132.34 µs 133.89 µs 135.50 µs]
                        change: [−1.3913% +0.2217% +1.9769%] (p = 0.79 > 0.05)
                        No change in performance detected.
```

### Play Sound For Duration of Zero And Change Theme
This benchmark is the same as the previous one, but this also changes the theme.
```
Evaluate CLI: "sound_themer --theme freedesktop play --duration 0 complete"
                        time:   [138.63 µs 140.47 µs 142.62 µs]
                        change: [−2.9476% −1.1450% +0.7655%] (p = 0.23 > 0.05)
                        No change in performance detected.
```
This benchmark takes ~7 microseconds longer than the previous benchmark, so it can be inferred that selecting a new theme is a very fast operation.

### List Sound Files In Theme Directories
This benchmark makes any `println!()` macros write to `std::io::sink()`, since otherwise the benchmark time would be dominated by the slow I/O writes.
```
Evaluate CLI: "sound_themer list"
                        time:   [29.932 µs 30.053 µs 30.181 µs]
                        change: [−0.8119% −0.3180% +0.1972%] (p = 0.23 > 0.05)
                        No change in performance detected.
```

<br/><br/>


## Requirements

* `wireplumber` (Pipewire) for playing sounds via `pw-play`
* `sound-theme-freedesktop` for a default theme
