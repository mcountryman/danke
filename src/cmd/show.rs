use crate::err::Error;
use crate::state::State;
use crate::{ops, yabai};

pub fn run() -> Result<(), Error> {
  let mut state = State::read()?;

  let windows = yabai::query_windows()?;
  let window = 'find: loop {
    let id = match state.front() {
      Some(id) => *id,
      None => return state.save(),
    };

    for window in &windows {
      if window.id == id {
        break 'find window;
      }
    }

    // Remove entries without an associated window
    state.pop_front();
  };

  ops::toggle(window)?;

  state.save()
}
