# SDL (Simple DirectMedia Layer)

https://github.com/Rust-SDL2

## Setup

```sh
# Mac
brew install sdl2

# Linux
sudo apt install libsdl2-dev
```

Add this line to your ~/.zshenv or ~/.bash_profile depending on whether you use ZSH or Bash.

```sh
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Dependency

```toml
sdl2 = "0.37.0"
```
