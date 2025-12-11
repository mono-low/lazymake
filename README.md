# lazymake

a modern, interactive tui for make/justfiles - inspired by lazyvim and lazygit. browse, search, and execute your build tasks with ease.

## features

interactive task browser
- browse all available targets/recipes with descriptions
- fuzzy search filtering for quick task discovery
- view task dependencies at a glance
- color-coded task information

task execution
- run tasks directly from the tui
- live output streaming in split panel
- task execution history tracking
- exit code visibility

dependency graph
- show dependency tree for the selected task
- detect simple cycles and mark them
- rendered as an indented text tree in the output panel

multi-format support
- full makefile support
- justfile support (just alternative to make)
- auto-detection of file type
- works with both formats seamlessly

beautiful tui
- inspired by lazyvim and lazygit
- responsive keyboard navigation
- clean, minimal interface
- dark/light terminal compatible

## installation

### from source

```bash
git clone https://github.com/yourusername/lazymake
cd lazymake
cargo build --release
./target/release/lazymake
```

### prerequisites

- rust 1.70+ (for building from source)
- a terminal with 256-color support
- make or just installed (for running tasks)

## usage

simply run `lazymake` in a directory containing a `makefile` or `justfile`:

```bash
cd your-project
lazymake
```

### keybindings

| key | action |
|-----|--------|
| `↑` / `↓` | navigate tasks |
| `/` | start fuzzy search filter |
| `p` | edit task parameters for the selected task |
| `g` | show dependency graph for the selected task |
| `pageup` / `pagedown` | page up/down (task list or output panel) |
| `home` / `end` | jump to first/last task |
| `esc` | cancel filter or parameter input |
| `enter` | execute selected task |
| `o` | toggle output panel |
| `h` | show task execution history |
| `?` | show help |
| `q` / `esc` | quit |

## example files

### makefile example

```makefile
# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy
```

### justfile example

```justfile
# Build the project
build:
    cargo build --release

# Run tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy
```

## architecture

lazymake is built with:

- **ratatui** - terminal ui framework
- **crossterm** - cross-platform terminal handling
- **tokio** - async runtime for task execution
- **fuzzy-matcher** - fuzzy search filtering
- **petgraph** - dependency graph utilities (future, beyond text tree)

### project structure

```
src/
├── main.rs       # application entry point
├── app.rs        # application state management
├── parser.rs     # makefile/justfile parsing
├── executor.rs   # task execution engine
└── tui.rs        # terminal ui rendering
```

## roadmap and future ideas

### done

- [x] parse makefiles and justfiles
- [x] interactive task browsing
- [x] fuzzy search filtering
- [x] task execution with detailed output
- [x] task history with timestamps
- [x] basic dependency view in task list
- [x] dependency graph visualization (text tree in output panel)
- [x] task parameter input (appended to commands)
- [x] empty state handling when no tasks are found
 - [x] output scrolling in output panel (pageup/pagedown)
 - [x] better filter feedback ("no tasks match" message)
 - [x] extra keyboard navigation (pageup/pagedown/home/end)

### planned features

- [ ] configuration and customization
  - .lazymake.toml for user and project settings
  - theme customization and color tweaks
  - save filter and parameter history

- [ ] custom keybindings
  - remap all actions (navigation, filter, params, graph, output)
  - presets for vim-style and emacs-style bindings

- [ ] output and dependency visualization
  - scrolling for long output
  - search inside the output panel
  - optional syntax highlighting for common output formats
  - ascii-style dependency graph view in addition to the text tree

- [ ] ci and automation
  - github actions workflow for automated builds
  - run tests, fmt, and clippy on each push
  - build on linux, macos, and windows
  - publish tagged releases automatically

- [ ] parser and executor robustness
  - better errors for malformed makefiles and justfiles
  - handle empty or missing files gracefully
  - validate task names and detect duplicates
  - timeouts for long-running commands
  - better signal handling for interrupted tasks
  - capture stderr separately from stdout where useful
  - graceful shutdown (ctrl+c handling, restore terminal state, optional state save)

- [ ] ui and ux improvements
  - show number of matching tasks in filter mode
  - clear "no matches" message when filter hides all tasks
  - better visual feedback while a task is running
  - loading indicator for long-running tasks
  - handle terminal resize events gracefully
  - keyboard improvements (page up/down, home/end, ctrl+a in filter)

- [ ] testing
  - unit tests for the parser on various makefile and justfile styles
  - integration tests for the executor
  - tui tests using a terminal test backend
  - edge case coverage (empty files, huge task lists, failing commands)

- [ ] documentation and onboarding
  - troubleshooting section for common issues
  - faq for typical questions
  - screenshots or asciinema-style demos
  - short video walkthrough of the main workflow

- [ ] quality and maintenance
  - remove remaining unused fields and imports
  - add doc comments to public functions and types
  - improve error messages where they are still generic
  - add structured logging hooks for debugging

## contributing

contributions are welcome! please feel free to submit a pull request.

## license

mit license - see license file for details

## inspiration

lazymake is inspired by:
- [lazyvim](https://www.lazyvim.org/) - neovim configuration
- [lazygit](https://github.com/jesseduffield/lazygit) - git tui
- [just](https://github.com/casey/just) - task runner
