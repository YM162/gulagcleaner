# Gulag Cleaner Rust Distribution
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/gulagcleaner_rs)

## Setting Up Rust

To incorporate Rust components within Gulag Cleaner, ensure Rust is correctly installed on your system. Follow the installation guide on the [official Rust website](https://www.rust-lang.org/tools/install) for detailed instructions. This includes installing `rustup`, which is the Rust toolchain manager, and the Rust compiler (`rustc`).

## Using `gulagcleaner_rs` in your Rust code

To use this crate in your code, please refer to the following documentation:
**https://docs.rs/gulagcleaner_rs/latest/gulagcleaner_rs/**

## Running Rust Tests

Gulag Cleaner leverages Rust for certain operations, providing performance and safety benefits. To ensure these components work as expected, comprehensive tests are included.

To run the tests:

1. Open a terminal.
2. Navigate to the root directory of Gulag Cleaner.
3. Execute the following command to run all tests:

   ```bash
   cargo test
   ```

   For more detailed test outputs, including print statements from your tests, use:

   ```bash
   cargo test --package gulagcleaner_rs --lib -- tests --nocapture
   ```

   This command targets the specific Rust package (`gulagcleaner_rs`) and enables detailed outputs with `--nocapture`.

   Note: at the moment this test only include the reading, cleaning and writing of 2 example PDFs for Wuolah and Studocu.
