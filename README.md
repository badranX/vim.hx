<h1 align="center">Helix-Vim-Mod</h1>

A [Helix](https://helix-editor.com) fork that adds Vim-like keybindings — intended as a lightweight patch, without altering the core functionality of Helix.
<br>

<p align="center">
  <img src="./screenshot.png" alt="Screenshot" style="width:80%;" />
</p>


## Installation
Rust has great tooling! You can build this repo from source just like Helix itself:
👉 [Follow the official Helix build guide](https://docs.helix-editor.com/building-from-source.html)

## Vim Supported Keybindings (Partial List)

### Visual Mode & Visual Lines

- `v`, `V`
- `va<char>`, `vi<textobject>` (`<textobject>`: `w`, `W`, `p`...etc)
- Treesitter-related selection such as `vaf` to select a function.
- `gv`

### Operators/Modifiers

- `d`, `dd`, `c`, `cc`, `y`, `yy` 
- `[c|y|d]<motion>`  like `dw`, `dB`
- `[c|y|d]{textobject}` like  `diw`, `da)`, `yi}`
-  Treesitter-related modification keybindings such as `daf` to delete a function or `yaf` to yank a function.

### Navigation

- `*`, `#`, `n`, `N`
- `0`, `^`, `$`
- `f<char>`, `F<char>`, `t<char>`, `T<char>`
- `{`, `}`
- `w`, `W`, `b`, `B`, `e`, `E`
- `gg`, `G`
- `C-^`, `C-6`

### 🔍 Things to Watch For
- No `Ctrl-R` for redo — Instead, use `U`, similar to Helix.
- The `%` key is mapped to `match_brackets`, similar to Vim. To revert this mapping or assign it to a custom key, update the Helix configuration as follows:

  ```toml
  [keys.normal]
  "%" = "select_all"
  [keys.select]
  "%" = "select_all"
  ```


## Alternatives / Similar Projects

Here are some other projects you might find interesting:

- [**helix-vim**](https://github.com/LGUG2Z/helix-vim) — A Vim-like configuration for Helix. This is an attempt to provide Vim-like keybindings using Helix configs only.
  
- [**evil-helix**](https://github.com/usagi-flow/evil-helix) — A fork of Helix that inspired this project.
