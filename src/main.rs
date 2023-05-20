mod config;
mod corner;
mod wayland;

use anyhow::Result;
use clap::Parser;
use config::get_configs;
use std::path::PathBuf;
use wayland::Wayland;

/// Hot corners for Wayland.
/// Waycorner allows you to create anchors on specified locations of your monitors and execute a command of your choice.
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Andreas Backx")]
struct Opts {
    /// Config file path.
    #[clap(short, long, default_value = "~/.config/waycorner/config.toml")]
    config: PathBuf,
    /// Preview the corners on your screen(s).
    #[clap(short, long)]
    preview: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    let mut frontend = Wayland::new(get_configs(opts.config)?, opts.preview);
    frontend.run()
}
