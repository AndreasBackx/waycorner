mod config;
mod corner;
mod wayland;

#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{AppSettings, Clap};
use config::get_configs;
use std::path::PathBuf;
use wayland::Wayland;

/// Hot corners for Wayland.
/// Waycorner allows you to create anchors on specified locations of your monitors and execute a command of your choice.
#[derive(Clap)]
#[clap(version = "0.1.4", author = "Andreas Backx")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Config file path.
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "~/.config/waycorner/config.toml"
    )]
    config: PathBuf,
    /// Preview the corners on your screen(s).
    #[clap(short, long)]
    preview: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::parse();

    let mut frontend = Wayland::new(get_configs(opts.config)?, opts.preview);
    frontend.run()
}
