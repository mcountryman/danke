# danke

A stash utility built for [yabai](https://github.com/koekeishiya/yabai).

danke will allow you to cycle through minimized windows, toggle the visibility of a minimized window,
and force them in a `floating` state until you're done with them.

## Install

```shell
git clone https://github.com/mcountryman/danke.git
cargo install --path danke
```

## Usage

```shell
# Cycles through stashed windows
danke cycle

# Toggles the visiblity of the last stashed window
danke show

# Changes the stashy-ness of the focused window
danke stash
```

## Examples

* [.skhdrc](examples/.skhdrc)
* [.yabairc](examples/.yabairc)
