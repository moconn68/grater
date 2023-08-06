# grater: A Rusty Mouse-Mover 🐭

## Disclaimer ⚠️

This tool is written as a hobby project for learning purposes only. I make no gurantees about its efficacy and hold no responsibility over any unintended consequences of you using this software (especially on your work computer!).

## About ℹ️

grater is a simple program that does one thing: moves your computer's mouse cursor around automatically. On a random interval (currently every 2-30 seconds), your mouse will be moved to a random location on your computer screen. This is useful in several instances, such as if you needed to prevent your computer from going idle/sleeping for some reason.

## Features 🔔

- Automatically moves your mouse to a random spot on your screen
- Runs indefinitely until you close the corresponding application window (or otherwise kill the process)
- Cursor location and pause between moves is randomized to dissuade curiosity

## Usage 🛠️

I plan to provide pre-built artifacts for Linux, MacOS, and Windows in the near future. However, for the time being, you must download the project source and build/run it yourself.

### Pre-requisites 💾

This program naturally requires the Rust programming language and its toolchain; if you haven't already, please install it: https://www.rust-lang.org/tools/install

### Installation ⚙️

1. Download this project via Git or your preferred method
2. To run directly from the project source, just do `cargo run` in the project directory
3. To install the binary to your system, use `cargo install --path <grater directory>`. Now you can just run `grater` from your terminal anywhere!

## Software used 👨‍💻

+ [`rand`](https://crates.io/crates/rand) - generating random numbers for cursor position and pause time
+ [`winit`](https://crates.io/crates/winit) - manages the main application window and lifecycle, as well as controlling the automated cursor movement