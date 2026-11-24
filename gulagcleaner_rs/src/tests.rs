use crate::clean::clean_pdf;
use crate::models::method::graphics_state_balance;
use lopdf::{dictionary, Document, Object, Stream};
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

/// Content streams of a page laid out like Wuolah does with pdf-lib: the original page
/// wrapped in `q cm` streams that make room for the ads, followed by the ads.
const WUOLAH_LAYOUT: [&[u8]; 17] = [
    // Ad layout: moves and shrinks the original page.
    b"q\n1 0 0 1 4.94 3.49 cm\n",
    b"q\n0.876 0 0 0.876 0 0 cm\n",
    b"q\n1 0 0 1 107.66 0 cm\n",
    // Original page.
    b"q\n1 0 0 1 0 0 cm\n",
    b"q\n1 0 0 1 0 0 cm\n",
    b"q\n",
    b"q 0 0 1 rg 20 20 200 100 re f BT /F1 12 Tf (Q \\) q) Tj ET Q",
    b"Q\n",
    b"",
    b"Q\n",
    b"Q\n",
    // Ads: logo, side banner and hidden text.
    b"q\n1 0 0 1 713.14 36 cm\n98.75 0 0 18 0 0 cm\n/Logo Do\nQ\n",
    b"Q\n",
    b"q\n1 0 0 1 -107.66 0 cm\n102.66 0 0 600.28 0 0 cm\n/Banner Do\nQ\n",
    b"Q\n",
    b"Q\n",
    b"q\nBT\n1 1 1 rg\n(hidden text) Tj\nET\nQ\n",
];

/// Streams of `WUOLAH_LAYOUT` that draw the original page.
const ORIGINAL_PAGE: std::ops::RangeInclusive<usize> = 3..=10;

/// Builds a one-page PDF of the given size whose page follows `WUOLAH_LAYOUT`.
fn wuolah_layout_pdf(width: f32, height: f32) -> Vec<u8> {
    let mut doc = Document::with_version("1.7");
    let pages_id = doc.new_object_id();
    let contents: Vec<Object> = WUOLAH_LAYOUT
        .iter()
        .map(|data| {
            doc.add_object(Stream::new(dictionary! {}, data.to_vec()))
                .into()
        })
        .collect();
    let image = |width: i64, height: i64| {
        let dict = dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width,
            "Height" => height,
            "ColorSpace" => "DeviceGray",
            "BitsPerComponent" => 8,
        };
        Stream::new(dict, vec![0; (width * height) as usize])
    };
    let logo_id = doc.add_object(image(40, 8));
    let banner_id = doc.add_object(image(24, 140));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![0.into(), 0.into(), width.into(), height.into()],
        "Contents" => contents,
        "Resources" => dictionary! {
            "XObject" => dictionary! {
                "Logo" => logo_id,
                "Banner" => banner_id,
            },
        },
    });
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {
            "Type" => "Pages",
            "Kids" => vec![page_id.into()],
            "Count" => 1,
        }),
    );
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut data = Vec::new();
    doc.save_to(&mut data).unwrap();
    data
}

/// A document with a single Wuolah page has no other page to compare it with, so it used to
/// fall back to the naive method, whose crops only fit portrait pages.
#[test]
fn test_single_page_wuolah_layout() {
    // A4 landscape and A4 portrait.
    for (width, height) in [(841.89, 595.28), (595.28, 841.89)] {
        let (clean_file, method_code) = clean_pdf(wuolah_layout_pdf(width, height), false);
        assert_eq!(
            method_code, 0,
            "{width}x{height}: expected the Wuolah method"
        );

        let doc = Document::load_mem(&clean_file).unwrap();
        let pages = doc.get_pages();
        assert_eq!(pages.len(), 1);
        let page_id = pages[&1];

        // Only the original page is left, without the ad layout and the ads.
        assert_eq!(
            doc.get_page_content(page_id).unwrap(),
            WUOLAH_LAYOUT[ORIGINAL_PAGE].concat()
        );

        // And it keeps its size and orientation.
        let media_box: Vec<f32> = doc
            .get_dictionary(page_id)
            .unwrap()
            .get(b"MediaBox")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_float().unwrap())
            .collect();
        for (value, expected) in media_box.iter().zip([0.0, 0.0, width, height]) {
            assert!((value - expected).abs() < 1e-3, "MediaBox {media_box:?}");
        }
    }
}

#[test]
fn test_graphics_state_balance() {
    assert_eq!(graphics_state_balance(b"q\n1 0 0 1 4.94 3.49 cm\n"), (1, 0));
    assert_eq!(graphics_state_balance(b"Q\n"), (-1, -1));
    assert_eq!(graphics_state_balance(b"Q q 1 0 0 RG"), (0, -1));
    assert_eq!(graphics_state_balance(b""), (0, 0));
    // `q` and `Q` inside strings, hex strings, comments and names are not operators.
    assert_eq!(
        graphics_state_balance(b"q BT (Q \\) (q)) Tj <51> Tj ET % Q\n/Q gs Q"),
        (0, 0)
    );
    // Neither are they inside inline image data.
    assert_eq!(
        graphics_state_balance(b"q BI /W 1 /H 1 /BPC 8 /CS /G ID Q\nEI Q"),
        (0, 0)
    );
}
