# FFmpeg with Rust

## Building FFmpeg for Windows

If you'd like the skip this step, I have a version pre-compiled for VS2019 [available here](https://mattrudder.com/libs/vs2019/FFmpeg-4.2.1.zip).

1. Install [msys2](https://www.msys2.org/)
    - Optional: Update `msys2_shell.cmd` to include the settings for native symlinks and inheriting PATH. Both are in the file already, but commented out by default:
        - `set MSYS=winsymlinks:nativestrict`
        - `set MSYS2_PATH_TYPE=inherit`
2. Launch a session with VS2019 Command Prompt settings
    - To start msys2 under Windows Terminal, use the following command: `C:\\msys64\\msys2_shell.cmd -defterm -no-start -mingw64`
    - You can generate a shell script with the appropriate env variables with envdiff `cargo install --git=https://gitlab.com/antekone/envdiff.git`
        - Launch `envdiff` under a clean `cmd` session
        - Launch `envdiff` again under VS2019 command prompt and the output will be your shell script
3. `mv /usr/bin/link.exe /usr/bin/link.exe.bak`
4. Update packages: `pacman -Syu`
5. Install FFmpeg dependencies: `pacman -S make diffutils yasm nasm`
6. Extract FFmpeg source to a working directory
7. Configure: `./configure --target-os=win64 --arch=x86_64 --enable-x86asm --toolchain=msvc --enable-shared`
8. Build and Install: `make && make install`
