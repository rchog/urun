# URun

**This is *super* pre-alpha and absolutely unusable!**

This is a small and mildly silly project I'm using to try to learn Rust, and as such is unlikely to be good or even usable for some time. Below is a blueprint for some of what I envision for it, rather than any kind of description of it's current state.

## What
A small, simple utility for launching applications from the desktop, designed to be used from WM keybindings, E.G. using `Sway`:

```shell
    # URun launcher
    bindsym Mod4+x exec urun
```

# How
It scans your `$PATH` directories for executable files and presents a list of suitable candidates as you type, and supports readline-style keybindings for selecting in the list.

# Why
I want a program like this, and the alternatives I found with a cursory search in my system's package manager all seemed far more complicated to use than they need to be.

# Development
- [x] Get GUI looking approximately right
- [x] Read all executables from PATH
    - [ ] Do it smart. (Cache found files with a timestamp, have a keybinding to force update)
- [x] List results in GUI
- ~~[x] Highlight results on mouseover~~ Can't do it with the way items are shown at the moment and still have access to `item.clicked()`. Mouse UX isn't top priority at the moment though.
- [x] Run hovered task on click
- [x] Keyboard navigation in list: `TAB` to move down list, `S-TAB` to move up, `RET` to launch. `C-n` and `C-p` to navigate, maybe `C-y` to launch?
    - [ ] If it's not an absolute bastard to get working well, make `TAB` based movement "fill in" the input line without re-generating the suggestions until the user keeps typing

At this point I'll consider it usable for my purposes, but here's some more goals that I may or may not get to:
- Make the GUI rats' nest "nicer" to work with
- "History" feature in the input line. `M-n` and `M-p` to browse
- CLI flags to opt in to features that contaminate the environment such as the history and cache files mentioned above
- Maybe some CLI flags to spit out some potentially useful information that can be inferred from the system that parses $PATH, just because it seems easy to do and I'm personally mildly interested:
    - Duplicate entries in $PATH
    - Duplicate executables throughout $PATH -- Which one is launched?
    - Numbers
- Alternate horizontal GUI? CLI flags to select the GUI orientation
