use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug)]
pub struct Config {
  pub engine: PathBuf,
  pub patched_engine: PathBuf,
  pub target_dir: PathBuf,
  pub dll: String,
}

impl Config {
  pub fn new(engine: PathBuf, patched_engine: PathBuf) -> Self {
    let target_dir = crate::project_root().join("target/i686-pc-windows-msvc/release");

    Self {
      engine,
      patched_engine,
      target_dir,
      dll: "norncore".into(),
    }
  }
}
