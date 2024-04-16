## SDL2 Tetris
Rust Programming By Example
https://github.com/PacktPublishing/Rust-Programming-By-Example


## Setup SDL2 on windows
copy files in SDL2-devel-2.24.2-VC.zip to
C:\Users\benjhuang\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib

## Setup SDL_image
https://github.com/libsdl-org/SDL_image/releases
copy file in SDL2_image-devel-2.6.2-VC.zip to 
C:\Users\benjhuang\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib

By default, you can't use the image module with sdl2, we need to activate it. 
```
 [features]
    default = ["sdl2/image"]
```