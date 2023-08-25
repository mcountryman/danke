use crate::err::Error;
use crate::ops;
use crate::state::State;
use crate::yabai::{self, Window};

pub fn run() -> Result<(), Error> {
  let mut state = State::read()?;
  let windows = yabai::query_windows()?;

  match cycle(&mut state, &windows) {
    Action::None => {}
    Action::Toggle(window) => ops::toggle(&window)?,
    Action::Unstash(window) => ops::unstash(&window)?,
    Action::Cycle { stash, unstash } => {
      ops::stash(&stash)?;
      ops::unstash(&unstash)?;
    }
  };

  state.save()
}

#[derive(Debug, PartialEq)]
enum Action {
  None,
  Toggle(Window),
  Unstash(Window),
  Cycle { stash: Window, unstash: Window },
}

fn cycle(state: &mut State, windows: &[Window]) -> Action {
  /// Hides control flow because `match ...` statements look ugly IMO
  macro_rules! some_or {
    ($opt:expr, $or: expr) => {
      match $opt {
        Some(x) => x,
        None => $or,
      }
    };
  }

  loop {
    if state.is_empty() {
      return Action::None;
    }

    if state.len() == 1 {
      let entry = some_or!(state.front(), break);
      let window = some_or!(find_window(&windows, *entry), break);

      return Action::Toggle(*window);
    }

    let curr = some_or!(state.pop_front(), break);
    let next = some_or!(state.front(), break);

    let mut curr_window = None;
    let mut next_window = None;

    for window in windows {
      if window.id == curr {
        curr_window = Some(window);
      } else if window.id == *next {
        next_window = Some(window);
      }
    }

    let curr_window = some_or!(curr_window, continue);
    let next_window = some_or!(next_window, {
      state.pop_front();
      state.push_front(curr);
      continue;
    });

    // If the first entry in the stash stack is currently stashed, we unstash it.
    if curr_window.is_minimized {
      state.push_front(curr);

      return Action::Unstash(*curr_window);
    }

    state.push_back(curr);

    return Action::Cycle {
      stash: *curr_window,
      unstash: *next_window,
    };
  }

  Action::None
}

fn find_window(windows: &[Window], id: u32) -> Option<&Window> {
  windows.iter().find(|w| w.id == id)
}

#[cfg(test)]
mod tests {
  use super::{cycle, Action};
  use crate::state::State;
  use crate::yabai::Window;

  #[test]
  fn none_given_empty() {
    let windows = vec![];
    let mut state = State::from(vec![]);

    assert_eq!(cycle(&mut state, &windows), Action::None);
  }

  #[test]
  fn none_given_one_without_window() {
    let window = Window {
      id: 420,
      ..Default::default()
    };

    let windows = vec![window];
    let mut state = State::from(vec![69]);

    assert_eq!(cycle(&mut state, &windows), Action::None);
  }
  #[test]

  fn toggle_given_one() {
    let window = Window {
      id: 69,
      ..Default::default()
    };

    let windows = vec![window];
    let mut state = State::from(vec![69]);

    assert_eq!(cycle(&mut state, &windows), Action::Toggle(window));
  }

  #[test]
  fn toggle_given_missing_first_window() {
    let window = Window {
      id: 1,
      ..Default::default()
    };

    let windows = vec![window];
    let mut state = State::from(vec![5, 4, 3, 2, 1]);

    assert_eq!(cycle(&mut state, &windows), Action::Toggle(window));
  }

  #[test]
  fn toggle_given_missing_second_window() {
    let window = Window {
      id: 1,
      ..Default::default()
    };

    let windows = vec![window];
    let mut state = State::from(vec![1, 2, 3, 4, 5, 6, 7]);

    assert_eq!(cycle(&mut state, &windows), Action::Toggle(window));
  }

  #[test]
  fn unstash_given_minimized() {
    let first = Window {
      id: 1,
      is_minimized: true,
      ..Default::default()
    };

    let second = Window {
      id: 2,
      ..Default::default()
    };

    let windows = vec![first, second];
    let mut state = State::from(vec![1, 2]);

    assert_eq!(cycle(&mut state, &windows), Action::Unstash(first));
    assert_eq!(state.len(), 2);
    assert_eq!(state.front(), Some(&1));
    assert_eq!(state.back(), Some(&2));
  }

  #[test]
  fn cycle_happy() {
    let first = Window {
      id: 1,
      ..Default::default()
    };

    let second = Window {
      id: 2,
      ..Default::default()
    };

    let windows = vec![first, second];
    let mut state = State::from(vec![1, 2]);

    assert_eq!(
      cycle(&mut state, &windows),
      Action::Cycle {
        stash: first,
        unstash: second
      }
    );

    assert_eq!(state.len(), 2);
    assert_eq!(state.front(), Some(&2));
    assert_eq!(state.back(), Some(&1));
  }

  #[test]
  fn cycle_given_missing_front_windows() {
    let first = Window {
      id: 1,
      ..Default::default()
    };

    let second = Window {
      id: 2,
      ..Default::default()
    };

    let windows = vec![first, second];
    let mut state = State::from(vec![10, 11, 12, 1, 2]);

    assert_eq!(
      cycle(&mut state, &windows),
      Action::Cycle {
        stash: first,
        unstash: second
      },
    );

    assert_eq!(state.len(), 2);
    assert_eq!(state.front(), Some(&2));
    assert_eq!(state.back(), Some(&1));
  }

  #[test]
  fn cycle_given_missing_second_windows() {
    let first = Window {
      id: 1,
      ..Default::default()
    };

    let second = Window {
      id: 10,
      ..Default::default()
    };

    let windows = vec![first, second];
    let mut state = State::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(
      cycle(&mut state, &windows),
      Action::Cycle {
        stash: first,
        unstash: second,
      }
    );

    assert_eq!(state.len(), 2);
    assert_eq!(state.front(), Some(&10));
    assert_eq!(state.back(), Some(&1));
  }
}
