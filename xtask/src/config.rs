use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Config {
  pub engine: PathBuf,
  pub patched_engine: PathBuf,
  pub target_dir: PathBuf,
  pub dll: String,
  pub wine: bool,
  pub dxwnd: bool,
  pub windows_path: Option<PathBuf>,
  pub x64dbg_path: Option<PathBuf>,
  pub dxwnd_path: Option<PathBuf>,
}

impl Config {
  pub fn new(engine: impl Into<PathBuf>, patched_engine: impl Into<PathBuf>) -> Self {
    let target_dir = crate::project_root().join("target/i686-pc-windows-msvc/release");

    let windows_path = std::env::var("WINDOWS_ENGINE_PATH").map(|p| p.into()).ok();
    let x64dbg_path = std::env::var("X64DBG_PATH").map(|p| p.into()).ok();
    let dxwnd_path = std::env::var("DXWND_PATH").map(|p| p.into()).ok();

    Self {
      engine: engine.into(),
      patched_engine: patched_engine.into(),
      target_dir,
      dll: "norncore".into(),
      wine: false,
      dxwnd: false,
      windows_path,
      x64dbg_path,
      dxwnd_path,
    }
  }
}
