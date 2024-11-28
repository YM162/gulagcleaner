use crate::models::method::Method;

use lopdf::Document;
use std::collections::HashSet;

/// Trait implemented by the different PDF methods
pub trait Cleaner {
    fn clean(&mut self, doc: &mut Document) -> (Vec<u32>, u8);
}

/// Cleans a PDF document by modifying its pages and removing unnecessary content.
///
/// # Arguments
///
/// * `data` - The PDF document data as a vector of bytes.
/// * `force_naive` - A boolean indicating whether to use the naive cleaning method.
///
/// # Returns
///
/// A tuple containing the cleaned PDF document data as a vector of bytes and a method code.
///
/// The method code indicates the cleaning method used: 0 for "Wuolah", 1 for "StuDocu", and 2 for "Naive".
pub fn clean_pdf(data: Vec<u8>, force_naive: bool) -> (Vec<u8>, u8) {

    //Load the PDF into a Document
    let mut doc = Document::load_mem(&data).unwrap();

    //We first need to determine what method we're using, either "Wuolah", "StuDocu" or "Wuolah naive".
    // We keep it like this to allow for future methods if needed.

    //Each method should mark pages for deletion in to_delete and modify the contents of the pages.

    let (to_delete, method_code) = match_method(&doc, force_naive).clean(&mut doc);

    //Delete the pages that we've marked for deletion.
    for (offset, page) in to_delete.into_iter().enumerate() {
        doc.delete_pages(&[page - offset as u32]);
    }
    //Save the document.
    let mut return_stream = Vec::new();
    doc.save_to(&mut return_stream).unwrap();

    // Should we still return the method_code now that we are going multi-language? I will leave it not returned for now.
    //return_stream.push(method_code);
    (return_stream, method_code)
    //doc.save_to("test.pdf").unwrap();
}

/// Creates a new `Method` instance based on the provided `Document` and `force_naive` flag.
///
/// # Arguments
///
/// * `doc` - A reference to the `Document` object.
/// * `force_naive` - A boolean flag indicating whether to force the use of the naive method.
///
/// # Returns
///
/// A `Method` instance representing the chosen method based on the provided `Document` and `force_naive` flag.
fn match_method(doc: &Document, force_naive: bool) -> Method {
    //0 for auto, 1 for wuolah, 2 for studocu 3 for wuolah naive
    if force_naive {
        return Method::Naive;
    }

    let pages = doc.get_pages();
    let content_list: Vec<Vec<(u32, u16)>> = pages
        .iter()
        .map(|x| doc.get_page_contents(*x.1))
        .filter(|x| x.len() > 1)
        .collect();
    let to_delete: Vec<u32> = pages
        .iter()
        .filter(|x| {
            let contents = doc.get_page_contents(*x.1);

            contents.len() < 4
        })
        .map(|x| *x.0)
        .collect();

    if content_list
        .iter()
        .map(|x| x.len())
        .filter(|x| *x == 3)
        .collect::<Vec<_>>()
        .len()
        == pages.len()
    {
        return Method::StuDocu(content_list);
    }
    let long_content_list: Vec<Vec<(u32, u16)>> = pages
        .iter()
        .map(|x| doc.get_page_contents(*x.1))
        .filter(|x| x.len() > 3)
        .collect();

    if long_content_list.len() > 1
        && long_content_list[0]
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&long_content_list[1].iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .len()
            > 1
    {
        return Method::Wuolah(long_content_list, to_delete);
    }
    Method::Naive
}
