use crate::err::{Error, ResultExt};
use miniserde::json;
use std::collections::VecDeque;
use std::env;
use std::fs::{self};
use std::io::ErrorKind;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

#[derive(Clone, Default)]
pub struct State(VecDeque<u32>);

impl State {
  /// Attempt to read the state from the state file in the home dir.  Will return [Default] if the
  /// file does not exist.
  pub fn read() -> Result<Self, Error> {
    match fs::read_to_string(Self::path()) {
      Ok(file) => {
        let stack = json::from_str::<Vec<u32>>(&file);
        let stack = stack.unwrap_or_default().into();

        Ok(Self(stack))
      }
      Err(err) if err.kind() == ErrorKind::NotFound => Ok(Default::default()),
      Err(_) => Err(Error::StateRead),
    }
  }

  /// Saves the state to the state file in the home dir.
  pub fn save(&self) -> Result<(), Error> {
    let stack = self.clone().make_contiguous().to_vec();
    let stack = json::to_string(&stack);

    fs::write(Self::path(), stack).ok_or(Error::StateWrite)?;

    Ok(())
  }

  fn path() -> PathBuf {
    let home = env::var("HOME").unwrap_or(".".to_string());
    let path = PathBuf::from(home);

    path.join(".danke.json")
  }
}

impl Deref for State {
  type Target = VecDeque<u32>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for State {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl From<Vec<u32>> for State {
  fn from(vec: Vec<u32>) -> Self {
    Self(vec.into())
  }
}
