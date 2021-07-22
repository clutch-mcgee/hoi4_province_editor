#![warn(missing_debug_implementations)]

pub mod app;
pub mod config;
pub mod error;
pub mod events;
pub mod util;

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
  better_panic::install();

  let root = root_dir().unwrap();
  env::set_current_dir(root).unwrap();

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

  let mut current_exe = env::current_exe()?.canonicalize()?;

  if current_exe.pop() {
    return Ok(current_exe);
  };

  Err(io::Error::new(io::ErrorKind::Other, "Failed to find an application root"))
}
