<h1 align="center">Vim.hx</h1>

A [Helix](https://helix-editor.com) patch that adds Vim-like keybindings. Ideal for users who prefer Vim motions but want to benefit from Helix’s editing capabilities like multi-cursor support and tree-sitter awareness.
<br>

<p align="center">
  <img src="./screenshot.png" alt="Screenshot" style="width:80%;" />
</p>

## Switching between Vim mode and Helix
Vim mode is enabled by default. Use `:vim_disable` to switch to Helix mode, and `:vim_enable` to switch back.

## Vim Supported Keybindings (Partial List)

### Visual Mode, Visual Lines, and Visual Block
- `v`, `V`
- `va<char>`, `vi<textobject>` (`<textobject>`: `w`, `W`, `p`...etc)
- Treesitter-related selection such as `vaf` to select a function.
- `gv`
 - Visual Block: `C-v` works similarly to Vim’s *visual block* mode, but it’s not exactly the same — It simply creates multiple cursors.

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


### 🔄 How to Find and Replace?
If you have `sed` on your System, you can use `:s/../../flags` like Vim or `:s|..|..|flags`. You don't need to add `%`, it will default to `:%s` in normal mode and will be applied to the selection in `visual` mode.

However, we advice using Helix multicursor to achive this:
1. **Select target text**
   - For the whole file: `ggVG`
   - You can also remap `select_all`/`vim_select_all` as explained earlier.

2. **Create multicursors**:
   - Press `s`, then type your regex (e.g., `foo`) and hit `<Enter>`. This will put a cursor on all `foo` in the buffer.

3. **Replace using multi-cursor**:  
   - Use Vim-style editing. For example, press `c` to change selection, then type your replacement text.

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


## 🔍 Things to Watch For
This project is not intended to be a replica of Vim, so note the following differences:

 - No `Ctrl-R` for redo — Instead, use uppercase `U`, as in Helix. You can remap it.
 - `s` is used by Helix for `select_regex` and it's an important command for multi-cursor support. Either use `c` instead of `s` or remap keys.
 - Some Helix commands behave differently in Vim mode (`:vim_enable`), especially those that create selections outside of `Select`/`Visual` mode. If you need any of these commands, wrap them with `vim_cmd_off` and `vim_cmd_on` in your config file:
  ```toml
  [keys.normal]
  "A-up" = ["vim_cmd_off", "expand_selection", "vim_cmd_on"]
  ```
 - Helix's `select_all` (`%`) is mapped to `match_brackets`, similar to Vim. You can remap it to `vim_select_all` which will work in both Vim and Helix mode.

 - Helix supports selections outside of "Select/Visual" mode. This patch does not change that behavior, as such selections are valuable for multi-cursor usage.

These differences might be reduced in the future.
