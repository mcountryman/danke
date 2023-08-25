use std::borrow::Cow;

pub enum Error {
  NoUserEnv,
  UnixConn,
  UnixWrite,
  UnixRead,
  Yabai(String),
  Json,
  StateRead,
  StateWrite,
}

impl Error {
  /// Gets the error message text.
  pub fn msg(&self) -> Cow<'static, str> {
    match self {
      Self::NoUserEnv => "Missing `$USER` env var".into(),
      Self::UnixConn => "Failed to connect to yabai socket `/tmp/yabai_$USER.socket`".into(),
      Self::UnixWrite => "Failed to write to yabai socket".into(),
      Self::UnixRead => "Failed to read from yabai socket".into(),
      Self::Yabai(msg) => {
        let mut err = String::with_capacity(msg.len() + 10);

        err.push_str("Yabai command failed: `");
        err.push_str(msg);
        err.push('\'');
        err.into()
      }
      Self::Json => "Failed to deserialize json".into(),
      Self::StateRead => "Failed to read `$HOME/.danke.json`".into(),
      Self::StateWrite => "Failed to write `$HOME/.danke.json`".into(),
    }
  }

  /// Gets the exit code for the error.
  pub fn exit_code(&self) -> i32 {
    match self {
      Self::StateRead => 66,                                   // EX_NOINPUT
      Self::UnixConn => 68,                                    // EX_NO_HOST
      Self::UnixRead | Self::UnixWrite | Self::Yabai(_) => 74, // EX_IOERR
      Self::Json => 76,                                        // EX_PROTOCOL
      Self::StateWrite => 77,                                  // EX_NOPERM
      Self::NoUserEnv => 78,                                   // EX_CONFIG
    }
  }
}

impl From<miniserde::Error> for Error {
  fn from(_: miniserde::Error) -> Self {
    Self::Json
  }
}

pub trait ResultExt<T> {
  fn ok_or(self, err: Error) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E> {
  fn ok_or(self, err: Error) -> Result<T, Error> {
    self.map_err(|_| err)
  }
}
