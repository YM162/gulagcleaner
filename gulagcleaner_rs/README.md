# Gulag Cleaner Rust Distribution

## Setting Up Rust

To incorporate Rust components within Gulag Cleaner, ensure Rust is correctly installed on your system. Follow the installation guide on the [official Rust website](https://www.rust-lang.org/tools/install) for detailed instructions. This includes installing `rustup`, which is the Rust toolchain manager, and the Rust compiler (`rustc`).

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

   Note: at the moment this test only include the reading, cleaning and writing of 2 example PDFs for Wuolah and Studocs.

## Rust Development Guidelines

To contribute to the Rust portion of Gulag Cleaner, please adhere to the following guidelines:

- **Code Clarity**: Write clear, readable code with meaningful variable names and concise functions.
- **Comments and Documentation**: Add comments explaining complex logic or important decisions. Update the `README.md` with relevant examples and instructions when adding new features or making significant changes.
- **Performance**: Optimize for efficiency. Rust is known for its performance, so ensure your contributions enhance or maintain the current speed and memory usage.
- **Testing**: Write tests for new features or bug fixes if possible. Ensure existing tests pass without modifications unless the changes are intended to update the test behavior.

## TODO for Rust

If you're looking to contribute, here are some areas that need attention:

- **Writing Tests**: Our test coverage could be improved. Writing additional unit and integration tests for the Rust code is a priority.
- **Documentation**: A detailed README.md needs to be added, including setup instructions, examples of usage, and a description of the functions available.
- **Code Optimization**: There's always room for performance improvements. Profiling and optimizing existing Rust code can significantly impact overall tool performance.

## Contributing

Contributions to the Rust codebase of Gulag Cleaner are highly encouraged. Whether you're fixing bugs, optimizing performance, or adding new features, your input is valued. Follow the project's contribution guidelines and submit pull requests with your changes.

## License

Gulag Cleaner is distributed under the GPL-3 license, which means it's open-source and free to use.
