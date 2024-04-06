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

