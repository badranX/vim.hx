<h1 align="center">Vim.hx</h1>

A [Helix](https://helix-editor.com) fork that adds Vim-like keybindings — intended as a lightweight patch, without altering the core functionality of Helix. Ideal for users who prefer Vim keybindings but want to benefit from Helix’s editing capabilities like multi-cursor support and tree-sitter awareness.
<br>

<p align="center">
  <img src="./screenshot.png" alt="Screenshot" style="width:80%;" />
</p>

## Switching between Vim and Helix
To switch between Vim mode and Helix, use

```bash
:vim_enable
```

```bash
:vim_disable
```

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

## 🔍 Things to Watch For
This project is not intended to be a replica of Vim, so note the following differences:

- No `Ctrl-R` for redo — Instead, use uppercase `U`, as in Helix.
 - Helix allows selections outside of "Select" mode (equivalent to Vim's "Visual" mode). Currently, this patch does not alter that behavior. The added Vim commands will ignore such selections.
 - `s` is used by Helix for `select_regex` and it's an important command for multi-cursor support, use `c` instead.
 - The `%` key is mapped to `match_brackets`, similar to Vim. To revert this mapping or assign it to a custom key, update the Helix configuration as follows:
 ```toml
  [keys.normal]
  "%" = "select_all"
  [keys.select]
  "%" = "select_all"
```

Some of these differences might be removed in the future.

### 🔄 How to Find and Replace?

1. **Select target text** using Vim motions:  
   - For the whole file: `ggVG`  
   - You can also remap `select_all` as explained earlier.

2. **Create multi-cursor by selecting using regex**:  
   - Press `s`, then type your regex (e.g., `(foo|bar)`) and hit `<Enter>`.

3. **Replace using multi-cursor**:  
   - Use Vim-style editing. For example, press `c` to change, then type your replacement text.

4. **Exit multi-cursor mode**:  
   - Press `,` (comma)


### 🗂️ Where’s the File Explorer?
 - `<Space>e`  Open file explorer in workspace root
 - `<Space>E`  Open file explorer at current buffer's directory
 - `<Space>f`  Open file picker
 - `<Space>F`  Open file picker at current working directory



## Installation
#### Build from Source
To get the latest, build this project from source—just like Helix itself.
👉 [Follow the official Helix build guide](https://docs.helix-editor.com/building-from-source.html)
#### Pre-built binaries
Download pre-built binaries from the [GitHub Releases page](https://github.com/badranX/vim.hx/releases/). Then, follow the [official Helix guide](https://docs.helix-editor.com/install.html#pre-built-binaries) for setup steps.
