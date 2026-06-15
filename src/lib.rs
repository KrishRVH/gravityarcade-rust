//! Headless core for the Macroquad port of `gravity_arcade.swf`.

#[cfg(target_family = "wasm")]
compile_error!(
    "gravityarcade is desktop-only; build the native executable with `cargo build --release`."
);

pub mod sim;
