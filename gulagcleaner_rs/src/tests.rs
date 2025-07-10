use crate::clean::clean_pdf;
use std::fs;
use std::time::Instant;

const OUT_PATH: &str = "example_docs/out";

/// Represents configuration for running a test, including the paths for input and output files.
struct TestConfig {
    input_path: &'static str,
    output_filename: &'static str,
}

/// Ensures the output directory exists, creating it if necessary.
/// This function is invoked before running tests to ensure a location
/// is available for storing cleaned PDFs.
fn create_output_directory() {
    fs::create_dir_all(OUT_PATH).expect("Failed to create output directory");
}

/// Reads a PDF from the specified path, cleans it, and returns the cleaned PDF data.
///
/// # Arguments
///
/// * `in_path` - A string slice that holds the path to the input PDF file.
///
/// # Returns
///
/// A `Result` which is `Ok` with a `Vec<u8>` containing the cleaned PDF data,
/// or an `Err` with a string describing the error.
fn read_and_clean_pdf(in_path: &str) -> Result<Vec<u8>, String> {
    let data =
        std::fs::read(in_path).map_err(|e| format!("Failed to read `{in_path}`: {e}"))?;
    let (clean_file, _) = clean_pdf(data, false);
    Ok(clean_file)
}

/// Writes the cleaned PDF data to a file in the output directory.
///
/// # Arguments
///
/// * `out_path` - The path where the cleaned PDF will be stored.
/// * `clean_file` - A vector of bytes representing the cleaned PDF data.
///
/// # Returns
///
/// A `Result` which is `Ok` if the file was successfully written, or an `Err`
/// with a string describing the error.
fn store_pdf(out_path: &str, clean_file: Vec<u8>) -> Result<(), String> {
    std::fs::write(out_path, clean_file)
        .map_err(|e| format!("Failed to write `{out_path}`: {e}"))
}

/// Executes a cleaning test using the provided `TestConfig`.
///
/// This function orchestrates the test process: creating the output directory,
/// cleaning the PDF specified in the `TestConfig`, and storing the cleaned PDF
/// in the output directory. It also measures and prints the duration of the test.
///
/// # Arguments
///
/// * `test_config` - A reference to the `TestConfig` containing the test parameters.
fn run_test_for_config(test_config: &TestConfig) {
    create_output_directory();

    let start = Instant::now();

    let clean_file = read_and_clean_pdf(test_config.input_path).expect("Failed to clean PDF");
    store_pdf(
        &format!("{}/{}", OUT_PATH, test_config.output_filename),
        clean_file,
    )
    .expect("Failed to store PDF");

    let duration = start.elapsed();

    println!(
        "Test for `{}` completed in {:?}",
        test_config.input_path, duration
    );
}

// Define tests for specific PDF files, utilizing the TestConfig structure.

#[test]
fn test_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-free-example.pdf",
        output_filename: "wuolah_clean.pdf",
    });
}

#[test]
fn test_010624_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-010624-example.pdf",
        output_filename: "wuolah_010624_clean.pdf",
    });
}

#[test]
fn test_170924_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-170924-example.pdf",
        output_filename: "wuolah-170924-example_clean.pdf",
    });
}

#[test]
fn test_280924_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-280924-example.pdf",
        output_filename: "wuolah-280924-example_clean.pdf",
    });
}

#[test]
fn test_280924_2_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-280924-2-example.pdf",
        output_filename: "wuolah-280924-2-example_clean.pdf",
    });
}

#[test]
fn test_300924_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-300924-example.pdf",
        output_filename: "wuolah-300924-example_clean.pdf",
    });
}

#[test]
fn test_300924_2_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-300924-2-example.pdf",
        output_filename: "wuolah-300924-2-example_clean.pdf",
    });
}

#[test]
fn test_031024_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-031024-example.pdf",
        output_filename: "wuolah-031024-example_clean.pdf",
    });
}

#[test]
fn test_031024_2_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-031024-2-example.pdf",
        output_filename: "wuolah-031024-2-example_clean.pdf",
    });
}

#[test]
fn test_041024_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-041024-example.pdf",
        output_filename: "wuolah-041024-example_clean.pdf",
    });
}

#[test]
fn test_061024_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-061024-example.pdf",
        output_filename: "wuolah-061024-example_clean.pdf",
    });
}

#[test]
fn test_191024_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-191024-example.pdf",
        output_filename: "wuolah-191024-example_clean.pdf",
    });
}

#[test]
fn test_281124_wuolah_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/wuolah-281124-example.pdf",
        output_filename: "wuolah-281124-example_clean.pdf",
    });
}

#[test]
fn test_studocu_pdf() {
    run_test_for_config(&TestConfig {
        input_path: "example_docs/studocu-example.pdf",
        output_filename: "studocu_clean.pdf",
    });
}