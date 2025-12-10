<h1 align="center">hypruler</h1>

<p align="center">
  📏 Measure anything on your screen. Built for Linux.
</p>

<p align="center">
  <img src="assets/demo.gif" alt="Hypruler demo" width="500"/>
</p>

---

## Installation

Build from source:

```bash
cargo build --release
cargo install --path .
```

## Usage

Add a keybind to your Hyprland config (`~/.config/hypr/hyprland.conf`):

```
bind = $mainMod, M, exec, hypruler
```

Or if you're using Omarchy (`~/.config/hypr/bindings.conf`):

```
bindd = SUPER, M, hypruler, exec, hypruler
```

## Requirements

- wlroots-based compositor (Hyprland, Sway, etc.)
- `wlr-screencopy-unstable-v1` protocol support

## License

MIT
