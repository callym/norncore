use crate::{config::Config, DynError};
use std::{fs, path::PathBuf};

pub fn find_index(config: &Config) -> Result<u32, DynError> {
  let path = config
    .dxwnd_path
    .as_ref()
    .unwrap()
    .with_file_name("dxwnd.ini");

  let file = fs::read_to_string(path)?;

  let engine = if let Some(path) = &config.windows_path {
    path.clone()
  } else {
    config.patched_engine.clone()
  };
  let engine = engine.to_str().unwrap();

  for line in file.lines() {
    if !line.starts_with("path") {
      continue;
    }

    let num = line.find("=").unwrap();
    let (num, path) = line.split_at(num);

    let (_, num) = num.split_at(4);
    let num = u32::from_str_radix(num, 10)?;

    let (_, path) = path.split_at(1);

    if path.eq_ignore_ascii_case(engine) {
      return Ok(num);
    }
  }

  Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "dxwnd.ini not found!",
  ))?
}
