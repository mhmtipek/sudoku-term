Sudoku game on your terminal! It generates board by itself, with 4 different difficulty levels.

![Screenshot](https://github.com/mhmtipek/sudoku-term/blob/7a1bd663c3cc29c683012780e5857f01f63a57e8/screenshot.png)

Use arrow keys to navigate, use num keys to set values. Initial values can't be changed. If there is a conflict, it'll be highlighted. Other keys are described on game screen.
Difficulty should be passed as argument. Here's --help output:

```
Usage: sudoku-term [OPTIONS] [DIFFICULTY]

Arguments:
  [DIFFICULTY]  Difficulty [default: medium] [possible values: easy, medium, hard]

Options:
      --hide-elapsed-time  Hide elapsed time
  -h, --help               Print help
  -V, --version            Print version

```

Installation:

- Arch Linux ([AUR](https://aur.archlinux.org/packages/sudoku-term)): `paru -S sudoku-term`

Thanks to projects :heart::

- [ratatui](https://github.com/ratatui/ratatui)
- [clap](https://github.com/clap-rs/clap)
- [crossterm](https://github.com/crossterm-rs/crossterm)
