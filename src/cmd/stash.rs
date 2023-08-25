use super::{StashArgs, StashBehavior};
use crate::err::Error;
use crate::ops;
use crate::state::State;
use crate::yabai::{self, Window};

pub fn run(args: StashArgs) -> Result<(), Error> {
  let state = State::read()?;
  let window = match find_window(args.window)? {
    Some(window) => window,
    None => return Ok(()),
  };

  let id = state.iter().position(|i| *i == window.id);

  match args.behavior {
    StashBehavior::Toggle => match id {
      Some(id) => restore(state, &window, id),
      None => stash(state, &window),
    },
    StashBehavior::Stash => stash(state, &window),
    StashBehavior::Unstash => match id {
      Some(id) => restore(state, &window, id),
      None => Ok(()),
    },
  }
}

fn stash(mut state: State, window: &Window) -> Result<(), Error> {
  ops::stash(&window)?;

  if !state.contains(&window.id) {
    state.push_front(window.id);
  }

  state.save()
}

fn restore(mut state: State, window: &Window, index: usize) -> Result<(), Error> {
  ops::restore(&window)?;

  state.remove(index);
  state.save()
}

fn find_window(id: Option<u32>) -> Result<Option<Window>, Error> {
  let windows = yabai::query_windows()?;

  Ok(match id {
    Some(id) => windows.into_iter().find(|w| w.id == id),
    None => windows.into_iter().find(|w| w.has_focus),
  })
}
