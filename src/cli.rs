use clap_flags;
use failure::ResultExt;
use std::io;
use std::path::PathBuf;
use structopt;

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(
  about = "Cross compilation template generator",
  raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub struct Cli {
  #[structopt(flatten)]
  logger: clap_flags::Log,
  #[structopt(flatten)]
  verbosity: clap_flags::Verbosity,
  /// Project name. Defaults to target directory name
  #[structopt(short = "n", long = "name")]
  name: Option<String>,
  /// Target directory
  #[structopt(default_value = ".")]
  dir: String,
}

impl Cli {
  /// Initialize a logger.
  #[inline]
  pub fn log(&self, name: &str) -> ::Result<()> {
    self
      .logger
      .log(self.verbosity.log_level(), name)
      .context(::ErrorKind::Log)?;
    Ok(())
  }

  /// Access the dir. Checks if it's a directory on disk.
  ///
  /// TODO: read the `cargo.toml` instead.
  #[inline]
  pub fn dir(&self) -> ::Result<PathBuf> {
    let path: PathBuf = self.dir.clone().into();
    if !path.is_dir() {
      let err = io::Error::new(io::ErrorKind::InvalidInput, "");
      Err(::ErrorKind::Io(err))?;
    }
    let path = path.canonicalize().context(::ErrorKind::Other)?;
    Ok(path)
  }

  /// Access the directory name.
  #[inline]
  pub fn name(&self) -> ::Result<String> {
    match &self.name {
      Some(name) => Ok(name.to_string()),
      None => {
        let dir = self.dir().context(::ErrorKind::Other)?;
        let dir = dir.iter().last().ok_or_else(|| ::ErrorKind::Other)?;
        let dir = dir.to_str().ok_or_else(|| ::ErrorKind::Other)?;
        Ok(dir.to_string())
      }
    }
  }
}
