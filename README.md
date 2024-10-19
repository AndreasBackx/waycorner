# waycorner

Hot corners for Wayland. Create anchors in the corners of your monitors and execute a command of your choice.

_Note: Currently only tested on swaywm. Though it should work on any Wayland compositor that supports the xdg-output protocol._

https://user-images.githubusercontent.com/1593486/120075347-cd7cc400-c098-11eb-9a76-7fb26ee9cab9.mp4

## Installation

If you would like waycorner to be available on your distro's package manager, feel free to make an issue if you have some time to help.

### Arch User Repository (AUR)

```zsh
paru -S waycorner
```

### Cargo (crates.io)

```zsh
cargo install waycorner --locked
```

### Nix/NixOS

[`waycorner` package]: https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/applications/misc/waycorner/default.nix
[search.nixos.org]: https://search.nixos.org/packages?channel=unstable&show=waycorner&from=0&size=50&sort=relevance&type=packages&query=waycorner
[@NotAShelf]: https://github.com/notashelf
[out-of-date package report]: https://github.com/NixOS/nixpkgs/issues/new?assignees=&labels=9.needs%3A+package+%28update%29&projects=&template=out_of_date_package_report.md&title=Update+request%3A+PACKAGENAME+OLDVERSION+%E2%86%92+NEWVERSION

There is a [waycorner package] package available via Nixpkgs. Refer to [search.nixos.org] page for Waycorner
for the appropriate installation methods.

> [!NOTE]
> The Waycorner package in Nixpkgs is not updated automatically by the project author, and is instead
> maintained by [@NotAShelf]. Please contact the maintainer or open an [out-of-date package report] in
> Nixpkgs.

### Manually

```zsh
git clone git@github.com:AndreasBackx/waycorner.git
cd waycorner
cargo install --path . --locked
```

## Configuration

Default configuration location is `~/.config/waycorner/config.toml`, this needs to be created manually. You can also specify a different location using `--config`. Preview your configuration with the `--preview` flag.

```toml
[left]
# Shell commands to execute when hotcorner is triggered,
# at least one is required.

# Command to run when cursor enters hotcorner.
# `command` is an alias for `enter_command`.
enter_command = [ "notify-send", "enter" ]
# Command to run when cursor exits hotcorner.
exit_command = [ "notify-send", "exit" ]

# Locations of the hot corners.
# Options:
# - for corners: top_left, top_right, bottom_right, and bottom_left;
# - for edges: top, bottom, right, left.
locations = ["bottom_right", "bottom_left"]  # default

# Size of the hot corners in pixels, for edges the size means the width
# for vertical edges, and height for horizontal edges. The other dimension
# will be the width/height of your display - the set margin.
size = 10  # default

# Margin on the sides of the hot edges, only applicable to edge locations.
# See the comment with sizes attribute above.
margin = 20  # default

# Timeout in milliseconds before command is triggered.
timeout_ms = 250  # default

# Hex color of the corner when previewed, supports transparency. (#AARRGGBB or #RRGGBB)
# (Useful for debugging purposes when setting up several hot corners.)
color = "#FFFF0000"  # default

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
