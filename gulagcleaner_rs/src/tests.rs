use crate::clean::clean_pdf;

#[test]
fn test_wuolah() {
    //Load some pdf bytes and clean it
    let data = std::fs::read("example_docs/wuolah-free-example.pdf").expect(
        "Missing Wuolah test PDF, please store one in path `example_docs/wuolah-free-example.pdf",
    );
    let (clean_pdf, _) = clean_pdf(data, false);
    //Stores the clean pdf in the out directory
    std::fs::write("example_docs/out/wuolah_clean.pdf", clean_pdf).unwrap();
}
#[test]

fn test_studocu() {
    //Stores the clean pdf in the out directory
    let data = std::fs::read("example_docs/studocu-example.pdf").expect(
        "Missing Studocu test PDF, please store one in path `example_docs/studocu-example.pdf",
    );
    let (clean_pdf, _) = clean_pdf(data, false);
    //Print the length of the pdf
    std::fs::write("example_docs/out/studocu_clean.pdf", clean_pdf).unwrap();
}
