use std::{
  env,
  fs,
  io::ErrorKind,
  path::{Path, PathBuf},
  process::Command,
};

use config::Config;

mod config;

type DynError = Box<dyn std::error::Error>;

fn main() {
  if let Err(e) = try_main() {
    eprintln!("{}", e);
    std::process::exit(-1);
  }
}

fn try_main() -> Result<(), DynError> {
  dotenv::dotenv()?;

  let task = env::args().nth(1);

  let config = config(env::args().skip(1))?;

  match task.as_deref() {
    Some("build") => build(&config)?,
    Some("run-wine") => run_wine(&config)?,
    _ => print_help(),
  }

  Ok(())
}

fn print_help() {
  eprintln!(
    "Tasks:
build      [engine]            bootstraps `norncore`, where `engine` is the path to your Docking Station's `engine.exe`
run-wine   [engine]            bootstraps then runs `norncore` with `wine`
"
  )
}

fn config(mut args: impl Iterator<Item = String>) -> Result<Config, DynError> {
  let path = args
    .nth(1)
    .or_else(|| std::env::var("NORNCORE_ENGINE").ok());

  let path = if let Some(path) = path {
    let path = PathBuf::from(path);

    if !path.is_file() {
      return Err(format!("{:?} is not a file", &path).into());
    }

    path
  } else {
    return Err("No path given.".into());
  };

  let patched = path.with_file_name("engine.exe");

  let path = path.with_file_name("engine.exe.orig");

  Ok(Config::new(path, patched))
}

fn run_wine(config: &Config) -> Result<(), DynError> {
  build(config)?;

  let wine = env::var("NORNCORE_WINE_BIN")
    .map(|p| format!("{}/wine", p))
    .unwrap_or("wine".into());

  let mut command = Command::new(wine);

  for (key, value) in std::env::vars() {
    if key.starts_with("NORNCORE_") {
      command.env(key.replace("NORNCORE_", ""), value);
    }
  }

  command
    .current_dir(config.patched_engine.parent().unwrap())
    .arg(&config.patched_engine)
    .spawn()?
    .wait()?;

  Ok(())
}

fn build(config: &Config) -> Result<(), DynError> {
  let remove = |path| match fs::remove_file(path) {
    Ok(_) => Ok(()),
    Err(err) => {
      if err.kind() != ErrorKind::NotFound {
        Err(err)
      } else {
        Ok(())
      }
    },
  };

  fs::copy(&config.patched_engine, &config.engine)?;

  let extensions = vec!["dll", "pdb"];

  for ext in extensions {
    let file = format!("{}.{}", config.dll, ext);

    remove(config.patched_engine.to_path_buf().with_file_name(file))?;
  }

  copy_lib(&config)?;

  remove(config.patched_engine.clone())?;

  patch_binary(&config)?;

  Ok(())
}

// Ideally this would be done in Rust, but I can't find a good library
// that lets you edit and re-save PE files
fn patch_binary(config: &Config) -> Result<(), DynError> {
  use m_pefile_rs::PeFileRe;

  let mut pe_file_new = PeFileRe::load_from_file(&config.engine)?;

  pe_file_new
    .add_import(format!("{}.dll", &config.dll), &["__dummy"])
    .unwrap();

  let bytes = pe_file_new.to_bytes();

  fs::write(&config.patched_engine, &bytes)?;

  Ok(())
}

fn copy_lib(config: &Config) -> Result<(), DynError> {
  let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
  let status = Command::new(cargo)
    .current_dir(project_root())
    .env(
      "RUSTFLAGS",
      std::env::var("MSVC_WINE_RUST")
        .map(|key| format!("-C linker={}", key))
        .unwrap_or("".into()),
    )
    .args(&["build", "--release", "--target", "i686-pc-windows-msvc"])
    .status()?;

  if !status.success() {
    return Err("cargo build failed".into());
  }

  let extensions = vec!["dll", "pdb"];

  for ext in extensions {
    let file = format!("{}.{}", config.dll, ext);

    let dst = config.target_dir.clone().join(&file);

    fs::copy(
      &dst,
      config.patched_engine.to_path_buf().with_file_name(file),
    )?;
  }

  Ok(())
}

fn project_root() -> PathBuf {
  Path::new(&env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
