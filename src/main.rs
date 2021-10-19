#![warn(missing_debug_implementations)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
pub mod util;
pub mod app;
pub mod config;
pub mod error;
pub mod events;
pub mod font;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

use crate::app::App;
use crate::events::launch;

use std::path::PathBuf;
use std::env;
use std::io;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

pub const APPNAME: &str = concat!("HOI4 Province Map Editor v", env!("CARGO_PKG_VERSION"));

fn main() {
  install_handler();

  let root = root_dir().expect("unable to find root dir");
  env::set_current_dir(root).expect("unable to set root dir");

  let opengl = OpenGL::V3_2;
  let screen = [WINDOW_WIDTH, WINDOW_HEIGHT];
  let mut window: GlutinWindow = WindowSettings::new(APPNAME, screen)
    .graphics_api(opengl).resizable(false).vsync(true)
    .build().expect("unable to initialize window");
  let mut gl = GlGraphics::new(opengl);
  launch::<App>(&mut window, &mut gl);
}

fn root_dir() -> io::Result<PathBuf> {
  if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
    return Ok(PathBuf::from(manifest_dir));
  };

  let mut current_exe = dunce::canonicalize(env::current_exe()?)?;

  if current_exe.pop() {
    return Ok(current_exe);
  };

  Err(io::Error::new(io::ErrorKind::Other, "failed to find an application root"))
}

fn install_handler() {
  use chrono::Local;
  use color_backtrace::{BacktracePrinter, Verbosity};
  use termcolor::NoColor;

  use std::fs::File;

  if cfg!(debug_assertions) {
    // Print to console if debug assertions are enabled
    let out = color_backtrace::default_output_stream();
    BacktracePrinter::new()
      .verbosity(Verbosity::Medium)
      .install(out);
  } else {
    // Dump to a file if debug assertions are disabled
    let printer = BacktracePrinter::new()
      .lib_verbosity(Verbosity::Medium);
    std::panic::set_hook(Box::new(move |pi| {
      let now = Local::now().format("%Y%m%d_%H%M%S");
      match File::create(format!("crash_{}.log", now)) {
        Ok(file) => if let Err(e) = printer.print_panic_info(pi, &mut NoColor::new(file)) {
          eprintln!("Error while printing panic: {:?}", e);
        },
        Err(e) => eprintln!("Error creating crash log: {:?}", e)
      };
    }));
  };
}
