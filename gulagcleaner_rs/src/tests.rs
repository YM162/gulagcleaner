use crate::clean::clean_pdf;
use std::fs;

const OUT_PATH: &str = "example_docs/out";

/// Creates out folder if missing so tests won't fail
fn create_out_folder() {
    fs::create_dir_all(OUT_PATH).unwrap();
}

#[test]
fn test_wuolah() {
    create_out_folder();

    //Load some pdf bytes and clean it
    let data = std::fs::read("example_docs/wuolah-free-example.pdf").expect(
        "Missing Wuolah test PDF, please store one in path `example_docs/wuolah-free-example.pdf",
    );
    let (clean_pdf, _) = clean_pdf(data, false);

    //Stores the clean pdf in the out directory
    std::fs::write(format!("{}/wuolah_clean.pdf", OUT_PATH), clean_pdf).unwrap();
}
#[test]

fn test_studocu() {
    create_out_folder();

    //Stores the clean pdf in the out directory
    let data = std::fs::read("example_docs/studocu-example.pdf").expect(
        "Missing Studocu test PDF, please store one in path `example_docs/studocu-example.pdf",
    );
    let (clean_pdf, _) = clean_pdf(data, false);
    //Print the length of the pdf
    std::fs::write(format!("{}/studocu_clean.pdf", OUT_PATH), clean_pdf).unwrap();
}
