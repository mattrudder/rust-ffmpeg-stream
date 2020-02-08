## Building FFmpeg for Windows
1. Install [msys2](https://www.msys2.org/)
2. Launch a session with VS2019 Command Prompt settings
    - To start msys2 under Windows Terminal, use the following command: `C:\\msys64\\msys2_shell.cmd -defterm -no-start -mingw64`
    - You can generate a shell script with the appropriate env variables with envdiff `cargo install envdiff`
        - Launch `envdiff` under a clean `cmd` session
        - Launch `envdiff` again under VS2019 command prompt and the output will be your shell script
3. `mv /usr/bin/link.exe /usr/bin/link.exe.bak`
4. Update packages: `pacman -Syu`
5. Install FFmpeg dependencies: `pacman -S make diffutils yasm nasm`
6. Extract FFmpeg source to a working directory
7. Configure: `./configure --target-os=win64 --arch=x86_64 --enable-x86asm --toolchain=msvc --enable-shared`
8. Build and Install: `make && make install`

