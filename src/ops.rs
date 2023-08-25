use crate::err::Error;
use crate::yabai::{self, Window};

pub fn toggle(window: &Window) -> Result<(), Error> {
  if window.is_minimized {
    unstash(window)
  } else {
    stash(window)
  }
}

pub fn stash(window: &Window) -> Result<(), Error> {
  if window.is_minimized {
    return Ok(());
  }

  yabai::send(["window", "--minimize", &window.id.to_string()])?;

  Ok(())
}

pub fn unstash(window: &Window) -> Result<(), Error> {
  yabai::send(["window", "--focus", &window.id.to_string()])?;

  if !window.is_floating {
    yabai::send(["window", "--toggle", "float"])?;
  }

  Ok(())
}

pub fn restore(window: &Window) -> Result<(), Error> {
  yabai::send(["window", "--focus", &window.id.to_string()])?;

  if window.is_floating {
    yabai::send(["window", "--toggle", "float"])?;
  }

  Ok(())
}
