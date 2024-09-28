# Grpe
Grpe is a 3d renderer for the terminal. It is completely CPU-based (unless your terminal is GPU-accelerated, of course) and does not rely on external libraries other than some platform-specific c-libraries.

## TODO
1. Rotation
2. I/O via raw input mode (extern c)
3. Signal handling (extern c)
4. Draw lines between vertices.
5. Create wrapper script to run with "cargo run --release --example spiral -- $(tput cols) $((($(tput lines) * 2 - 5)))"

## Note
mate-terminal --full-screen --hide-menubar -e "target/release/grpe -m spiral -i"

47.5 * x = 31.5
