# cfg
A configuration manager written in bash.

## How it works
1. Configuration directory is created (`$HOME/.config/cfg`) if non-existent.
2. If configuration (`$HOME/.config/cfg/cfg`) is non-existent, ask user to configure cfg (but default the editor to `$EDITOR` anyways).
3. When adding a new config to manage, it is saved to the config dir under the name of "app" or "app-type", with the contents:
```
app=program name
type=type of config
file=/path/to/file
```
4. Listing tracked configs is as simple as `ls $HOME/.config/cfg` since all files are named as the program and optionally contain an appended config type.
5. When configurations are backed up, they are copied to `$HOME/.local/share/cfg`.

## Implemented / Planned
What is implemented:
- Adding configs to manage
- Viewing / Editing configs
- Backing up configs
- Files named "program name - type of config" (if applicable) to avoid collision of two configs from same program, examples:
1. If the file is named `mpv`, it can be assumed that this is the main config or that no other types of configs are to be tracked.
2. If the file is named `mpv-input`, it can be assumed that this is a configuration that sets the input or keybindings for `mpv`.

Naming them this way allows you to manage multiple configs for the same program while also categorizing them at the same time.

What is planned:
- Removing configs (from either being managed or the filesystem)
- Version control(?) -- I wouldn't be too interested, but I'm sure others would be interested in this if I implemented it
