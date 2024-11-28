# Micro Apple
Playing Bad Apple on a BBC micro:bit

## Running
This whole project is written in rust, so you will need the rust toolchain (this was developed on `rustc 1.85.0-nightly (dff3e7ccd 2024-11-26)`)

### Using `micro_apple_build`
1. Install the ffmpeg development libraries for your platform (follow [these instructions](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building)) 
2. Get a copy of the bad apple video, and place it at `./badapple.mp4` (if you don't have an mp4 file, just edit `./micro_apple_build/src/main.rs` to look a different path)
3. In `micro_apple_build`, run `cargo run`
4. You should then have `./bad_apple_rle.bin` appear

### Flashing to a micro:bit
1. Plug in your micro:bit
2. Install the `thumbv7em-none-eabihf` target (`rustup target add thumbv7em-none-eabihf`)
3. Install [`probe-rs`](https://probe.rs/) and `flip-link` (`cargo install flip-link`)
4. If you're building within Windows subsystemm for linux, follow [the setup section of this](https://github.com/ierturk/rust-embedded-wsl-probe-rs?tab=readme-ov-file#setup) to get usbipd setup.
5. If you're on linux, setup `udev` rules to allow non-superusers to program the microbit using [this](https://docs.rust-embedded.org/discovery/microbit/03-setup/linux.html#udev-rules) (n.b. there is a more comprehensive set of rules in [the `probe-rs` docs](https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules), which I personally use)
6. In `micro_apple`, run `cargo embed --release --target thumbv7em-none-eabihf`. This will flash to the micro:bit. (n.b. there might be a warning that mentions that the core might be halted, I'm not really sure what causes it but you can ignore it).
7. Micro:bit will now play bad apple