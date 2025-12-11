# lazymake installation

## quick install (recommended)

### with cargo (easiest)

```bash
cargo install lazymake
```

that's it! lazymake is now available globally:

```bash
lazymake
```

## manual install

### from source

```bash
git clone https://github.com/yourusername/lazymake
cd lazymake
cargo build --release
cp target/release/lazymake ~/.local/bin/
```

### with homebrew (coming soon)

```bash
brew install lazymake
```

## prerequisites

- rust 1.70+ (only for building from source)
- a terminal with 256-color support
- make or just installed (for running tasks)

## verify installation

```bash
lazymake --help
```

or run it in any directory with a makefile or justfile:

```bash
cd your-project
lazymake
```

## what lazymake looks for

lazymake automatically detects and uses:
- `justfile` (preferred)
- `makefile` 
- `Makefile`

if no file is found, it will show a helpful error message.

## uninstall

### if installed with cargo

```bash
cargo uninstall lazymake
```

### if installed manually

```bash
rm ~/.local/bin/lazymake
```

## troubleshooting

### command not found

if you get `command not found: lazymake` after installing:

1. check your path:
   ```bash
   echo $PATH | grep -o "[^:]*bin[^:]*"
   ```

2. if using cargo install, ensure cargo's bin directory is in your path:
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```
   add this to your `~/.zshrc` or `~/.bashrc`.

### no tasks found

make sure you're in a directory with:
- a `justfile`, or
- a `makefile`/`Makefile`

and that the file contains at least one task/recipe.

### terminal colors

if colors look wrong, ensure your terminal supports 256 colors. most modern terminals do.

## next steps

once installed, check out the [readme](readme.md) for usage and keybindings.
