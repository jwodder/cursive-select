[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![CI Status](https://github.com/jwodder/cursivelect/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/cursivelect/actions/workflows/test.yml)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.95-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/cursivelect.svg)](https://opensource.org/licenses/MIT)

[GitHub](https://github.com/jwodder/cursivelect) | [Issues](https://github.com/jwodder/cursivelect/issues)

This is an experimental Rust library made using
[`cursive`](https://crates.io/crates/cursive) for presenting a user with a
full-screen terminal interface consisting of a series of *selection lists*
(radio buttons and sets of checkboxes) and capturing their choices.

Usage, in brief:

- Create a new `Curselect`

- Call `curselect.add(list_key, selector)` with each `RadioSelector` or
  `MultiSelector` you wish to present to the user

- Call `curselect.run()` to run the interface and get the user's selections.
  If the user cancelled/quit the interface, this method returns `None`;
  otherwise, it returns `Some(selections)`, where `selections` is a `Vec<(T,
  Selection)>` pairing the `list_key`'s supplied to `add()` with the choices
  the user made for each respective selection list.

The terminal interface supports the following keybindings:

| Key                                        | Command                                                                    |
| ------------------------------------------ | -------------------------------------------------------------------------- |
| <kbd>j</kbd>, <kbd>Down</kbd>              | Move down one item                                                         |
| <kbd>k</kbd>, <kbd>Up</kbd>                | Move up one item                                                           |
| <kbd>h</kbd>, <kbd>Left</kbd>              | Move left one item                                                         |
| <kbd>l</kbd>, <kbd>Right</kbd>             | Move right one item                                                        |
| <kbd>w</kbd>, <kbd>Page Up</kbd>           | Scroll up a page                                                           |
| <kbd>z</kbd>, <kbd>Page Down</kbd>         | Scroll down a page                                                         |
| <kbd>g</kbd>, <kbd>Home</kbd>              | Go to first item                                                           |
| <kbd>G</kbd>, <kbd>End</kbd>               | Go to last item                                                            |
| <kbd>Tab</kbd>                             | Go to start of next selection list or next button, wrapping around         |
| <kbd>Shift</kbd>+<kbd>Tab</kbd>            | Go to start of previous selection list or previous button, wrapping around |
| <kbd>Enter</kbd>, <kbd>Space</kbd>         | Select/toggle/activate current selection                                   |
| <kbd>q</kbd>, <kbd>Q</kbd>, <kbd>Esc</kbd> | Cancel & quit                                                              |

Mouse interactions are also supported.

See Also
========

- [`curselect`](https://github.com/jwodder/curselect), a roughly-equivalent
  implementation in Python made using the [urwid](https://urwid.org) library
