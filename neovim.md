# **NEOVIM CHEAT SHEET FOR PROGRAMMERS**

| **ACTION**                     | **COMMAND**                     | **DESCRIPTION**                                   |
|--------------------------------|---------------------------------|---------------------------------------------------|
| **BASIC NAVIGATION**           |                                 |                                                   |
| Move to line start             | `0`                             | Cursor to the beginning of the current line       |
| Move to line end               | `$`                             | Cursor to the end of the current line             |
| Move to next word              | `w`                             | Jump to the start of the next word                |
| Move to previous word          | `b`                             | Jump to the start of the previous word            |
| Go to specific line            | `:n`                            | Jump to line `n`                                  |
| **INSERT MODE**                |                                 |                                                   |
| Enter insert mode              | `i`                             | Insert before the cursor                          |
| Append text                    | `a`                             | Insert after the cursor                           |
| New line below                 | `o`                             | Open a new line below and enter insert mode       |
| New line above                 | `O`                             | Open a new line above and enter insert mode       |
| **EDITING**                    |                                 |                                                   |
| Delete character               | `x`                             | Delete the character under the cursor             |
| Delete word                    | `dw`                            | Delete from the cursor to the end of the word     |
| Delete line                    | `dd`                            | Delete the current line                           |
| Copy line                      | `yy`                            | Yank (copy) the current line                      |
| Paste                          | `p`                             | Paste after the cursor                            |
| Undo                           | `u`                             | Undo last change                                  |
| Redo                           | `Ctrl + r`                      | Redo last undone change                           |
| Replace                        | `r<char>`                       | Replace the character under the cursor            |
| **SEARCH & REPLACE**           |                                 |                                                   |
| Search text                    | `/text`                         | Search forward for `text`                         |
| Search backward                | `?text`                         | Search backward for `text`                        |
| Next occurrence                | `n`                             | Jump to the next search match                     |
| Previous occurrence            | `N`                             | Jump to the previous search match                 |
| Replace globally               | `:%s/old/new/g`                 | Replace `old` with `new` in the whole file        |
| Replace in range               | `:n,m s/old/new/g`              | Replace `old` with `new` between lines `n` and `m`|
| **WINDOW MANAGEMENT**          |                                 |                                                   |
| Split window horizontally      | `:split` or `:sp`               | Open a horizontal split                           |
| Split window vertically        | `:vsplit` or `:vsp`             | Open a vertical split                             |
| Move to next window            | `Ctrl + w, w`                   | Cycle through open windows                        |
| Close current window           | `:q`                            | Close the current split                           |
| Resize window                  | `Ctrl + w, <` or `Ctrl + w, >`  | Resize window horizontally                        |
| **CODE NAVIGATION**            |                                 |                                                   |
| Jump to definition             | `gd`                            | Go to the definition of the symbol under cursor   |
| Jump to declaration            | `gD`                            | Go to the declaration of the symbol under cursor  |
| List buffers                   | `:ls`                           | List all open buffers                             |
| Switch buffer                  | `:buffer n`                     | Switch to buffer `n`                              |
| **PLUGINS (E.G., LSP, GIT)**   |                                 |                                                   |
| Open file tree (NvimTree)      | `:NvimTreeToggle`               | Toggle the file explorer                          |
| LSP hover                      | `K`                             | Show hover documentation                          |
| LSP diagnostic                 | `:LspDiagnostics`               | Show all diagnostics                              |
| Git status                     | `:Git`                          | Show git status                                   |
| Git blame                      | `:Git blame`                    | Show blame for current line                       |
| **EXITING**                    |                                 |                                                   |
| Save                           | `:w`                            | Write (save) the current file                     |
| Quit                           | `:q`                            | Quit the current window                           |
| Save and quit                  | `:wq` or `ZZ`                   | Save the file and exit                            |
| Quit without saving            | `:q!`                           | Exit without saving changes                       |
