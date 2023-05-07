# wiper
simple program for overwriting free disk space, written in rust

Usage and installation:
```
git clone --depth=1 https://github.com/154pinkchairs/wiper
cd wiper && cargo build --release
./target/release/wiper <number of rounds> <true|false for using zeros, which is faster, but less secure than more random data>
