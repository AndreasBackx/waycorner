# waycorner

Hot corners for Wayland. Create anchors in the corners of your monitors and execute a command of your choice.

_Note: Currently only tested on swaywm. Though it should work on any Wayland compositor that supports the xdg-output protocol._

https://user-images.githubusercontent.com/1593486/120075347-cd7cc400-c098-11eb-9a76-7fb26ee9cab9.mp4

## Installation

If you would like waycorner to be available on your distro's package manager, feel free to make an issue if you have some time to help.

### Arch User Repository (AUR)

```zsh
yay -S waycorner
```

### Manually

```zsh
git clone git@github.com:AndreasBackx/waycorner.git
cd waycorner
cargo install --path .
```

## Configuration

Default configuration location is `~/.config/waycorner/config.toml`, this needs to be created manually. You can also specify a different location using `--config`. Preview your configuration with the `--preview` flag.

```toml
[left]
# Shell command to execute when hotcorner is triggered.
command = "lock"  # required

# Locations of the hot corners.
# Options: top_left, top_right, bottom_right, and bottom_left.
locations = ["bottom_right", "bottom_left"]  # default

# Alternatively, you can specify an edge.
# Options: left, right, top, bottom
# locations = ["top", "bottom"]

# Size of the hot corners in pixels. Size can be either one "size" value,
# or separate "size_height" and "size_width" values. Use one or the other.
size = 10  # default
# size_height = 10
# size_width = 10


# Timeout in milliseconds before command is triggered.
timeout_ms = 250  # default

# Optional output config to specify what output to use.
[left.output]
# Regex to match output descriptions on.
# Regex engine is similar to RE2: https://github.com/rust-lang/regex
#
# Use `swaymsg -t get_outputs` to get a list of outputs in the format:
# Output ${NAME} '${DESCRIPTION}'
description = ""  # default, empty means all outputs
```

Then add `exec waycorner` to your swaywm config.

## Logging

Pass `RUST_LOG` with either `trace`, `debug`, `info`, `warn`, or `error`. To set the logging level, default is `error`. See [env_logger documentation](https://docs.rs/env_logger/0.8.3/env_logger/).
