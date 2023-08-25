mod cmd;
mod err;
mod ops;
mod state;
mod yabai;

use cmd::{Args, Cmd};
use std::process;

fn main() {
  let args = argh::from_env::<Args>();
  let result = match args.cmd {
    Cmd::Cycle(_) => cmd::cycle::run(),
    Cmd::Show(_) => cmd::show::run(),
    Cmd::Stash(args) => cmd::stash::run(args),
  };

  if let Err(err) = result {
    let msg = err.msg();
    let code = err.exit_code();

    println!("{msg}");
    process::exit(code);
  }
}
