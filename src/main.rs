extern crate hidapi;

use keyboard::fnlock;
use log::debug;
use std::env;

use clap::Parser;

mod keyboard;

#[cfg(feature = "gui")]
mod gui;

fn api_main(opts: &Opts) {
  let to_be_locked = !opts.unlock;
  keyboard::find_device().map(|k380| fnlock(k380, to_be_locked));
}

#[derive(Parser, Clone, Debug, Default)]
#[command(about)]
pub struct Opts {
  #[arg(long)]
  unlock: bool,

  #[arg(long)]
  gui: bool,
}

fn main() {
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info")
  }
  env_logger::init();
  let opts = Opts::parse();
  debug!("{:?}", opts);
  if opts.gui {
    #[cfg(feature = "gui")]
    {
      gui::gui_main(opts.clone());
    }
  } else {
    api_main(&opts);
  }
}
