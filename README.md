# wandr

`wandr` (short "watch randr") is a tool which runs in the background and watches for changes to the connected displays.
This is done by polling the connected displays in a set interval by using the `RandR` X11 Extension.

This is useful f.e if you want to rearrange your Infobars or workspaces (as seen in the example configuration).

### Requirements

Your system needs to run the X Window System.

## Getting Started

Configuration is done by placing a `wandr.toml` file in the `$XDG_CONFIG_HOME` directory.
The path should usually be: `$HOME/.config/wandr/wandr.toml`.
The configuration file is structured by defining multiple setups.
An example configuration is provided in the repository.

```toml
[[setups]]
name = "Desk"
on = ["eDP1", "DP1"]
off = []
exec = [
  "MONITOR=DP1 $HOME/.scripts/bar.sh &",
  "bspc monitor eDP1 -d 1 2",
  "bspc monitor DP1 -d 3 4 5",
]
```

This setup is called "Desk", which is a name that can be freely choosen, but must be unique.

`on` and `off` is a list of the display identifiers which need to be connected and disconnected.

If both given lists match the actual display configuration of a system, the setup is activated.
In case multiple setups match, the first defined setup is used. 

Once a setup is activated a couple of commands, as defined in `exec` will be executed once.

You can get a list of your display identifiers by using `xrandr -q`.

`wandr` does not take care of configuring your displays in a certain order. This can be done by using the `xrandr` tool
and added as a command to the `exec` option.

This might be a feature in the future though.

### Logging

Additional output can be received by setting the `RUST_LOG` environment to `debug`.

Each setup change will be logged in the form of `(NewSetupName <- OldSetupName)`.

### Running

You can add `wandr &` to your `$HOME/.xinitrc` so it will be executed on startup.
