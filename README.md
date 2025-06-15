<h1 align="center">Helix-Vim-Mod</h1>

A [Helix](https://helix-editor.com) fork that adds Vim-like keybindings — intended as a lightweight patch or mod, without altering the core functionality of Helix.
> ⚠️ **Note**: This is a work in progress. Contributions and feedback are welcome!
<br>
<p align="center">
  <img src="./screenshot.png" alt="Screenshot" style="width:80%;" />
</p>


## Installation
Don’t be scared — Rust has great tooling! You can build this repo from source just like Helix itself:
👉 [Follow the official Helix build guide](https://docs.helix-editor.com/building-from-source.html)


## Vim Command Support

### Navigation

- `0`, `^`, `$`
- `w`, `W`, `b`, `B`, `e`, `E`
- `{`, `}`
- `gg`, `G`
- `*`, `#`, `n`, `N`
- `C-^`, `C-6`
- `f<char>`, `F<char>`, `t<char>`, `T<char>`

### Selection / Visual Mode

- `v`, `V`
- `va<char>`, `vi<textobject>` (`<textobject>`: `w`, `W`, `p`...etc)
- Treesitter Helix features inherated from Helix such as: `vif` to select inside a function.

### Operators/Modifiers

- `d`, `dd`, `d<motion>`, `d{textobject}`
- `c`, `cc`, `c<motion>`, `c{textobject}`
- `y`, `yy`, `y<motion>`, `y{textobject}`

### Insertion / Open Lines

- `o`, `O`
- `C` (change to end of line)

### Search

- `*`, `#`
- `/`, `?`

