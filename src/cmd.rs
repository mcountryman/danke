pub mod cycle;
pub mod show;
pub mod stash;

use argh::FromArgs;
use std::str::FromStr;

/// Yabai stashing utility.
#[derive(FromArgs)]
pub struct Args {
  #[argh(subcommand)]
  pub cmd: Cmd,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Cmd {
  Cycle(CycleArgs),
  Show(ShowArgs),
  Stash(StashArgs),
}

/// Cycles through stashed windows.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "cycle")]
pub struct CycleArgs {}

/// Toggles the visibility of the last stashed window.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "show")]
pub struct ShowArgs {}

/// Changes the stashy-ness of a window.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "stash")]
pub struct StashArgs {
  /// the id of the window to stash (defaults to the focused window)
  #[argh(option)]
  pub window: Option<u32>,
  /// the stash behavior (defaults to `toggle`)
  ///
  /// * `toggle` - Toggles the stashy-ness
  /// * `stash` - Add to the stash and minimize
  /// * `unstash` - Remove from the stash and re-tiles the window
  #[argh(option, default = "Default::default()")]
  pub behavior: StashBehavior,
}

#[derive(Debug, Default)]
pub enum StashBehavior {
  #[default]
  Toggle,
  Stash,
  Unstash,
}

impl FromStr for StashBehavior {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "toggle" => Ok(Self::Toggle),
      "stash" => Ok(Self::Stash),
      "unstash" => Ok(Self::Unstash),
      val => Err(format!("Unexpected behavior `{val}`")),
    }
  }
}
