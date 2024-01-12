use crate::models::{
    self,
    method::{find_iobj_pairs, remove_logo, Method},
};
use models::{method, page_type};

use lopdf::{Document, Object};
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
/// The method code indicates the cleaning method used: 0 for "Wuolah", 1 for "StuDocu", and 2 for "Naive".
pub fn clean_pdf(data: Vec<u8>, force_naive: bool) -> (Vec<u8>, u8) {
    //Load the PDF into a Document
    let mut doc = Document::load_mem(&data).unwrap();
    let pages = doc.get_pages();

    //We first need to determine what method we're using, either "Wuolah", "StuDocu" or "Wuolah naive".
    // We keep it like this to allow for future methods if needed.

    //Each method should mark pages for deletion in to_delete and modify the contents of the pages.

    let (to_delete, method_code) = match match_method(&doc, force_naive) {
        method::Method::Wuolah(content_list, to_delete) => {
            let new_contents: Vec<Vec<(u32, u16)>> = content_list
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let pares = if x == content_list.last().unwrap() {
                        find_iobj_pairs(x, &content_list[i - 1])
                    } else {
                        let check_if_00 = find_iobj_pairs(x, &content_list[i + 1]);
                        if check_if_00 != (0, 0) {
                            check_if_00
                        } else {
                            find_iobj_pairs(x, &content_list[i - 1])
                        }
                    };

                    x[(pares.0) - 2..=(pares.1) + 3].to_vec()
                })
                .collect();

            let vector: Vec<(&u32, &(u32, u16))> = pages
                .iter()
                .filter(|x| doc.get_page_contents(*x.1).len() > 1)
                .collect();
            for (i, page) in vector.iter().enumerate() {
                let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                let contents_objects: Vec<Object> = new_contents[i]
                    .iter()
                    .map(|x| Object::Reference(*x))
                    .collect();

                mutable_page.set(*b"Contents", lopdf::Object::Array(contents_objects));

                mutable_page.set("Annots", Object::Array(vec![]));
                let mediabox = mutable_page.get(b"MediaBox").unwrap().as_array().unwrap();

                let height_offset = match mediabox[1].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[1].as_i64().unwrap() as f32,
                };
                let width_offset = match mediabox[0].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[0].as_i64().unwrap() as f32,
                };

                let height = match mediabox[3].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[3].as_i64().unwrap() as f32,
                };
                let width = match mediabox[2].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[2].as_i64().unwrap() as f32,
                };

                for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                    mutable_page.set(
                        _box,
                        Object::Array(vec![
                            Object::Real(0.0),
                            Object::Real(0.0),
                            Object::Real(width - width_offset),
                            Object::Real(height - height_offset),
                        ]),
                    );
                }
            }

            (to_delete, 0)
        }

        method::Method::StuDocu(content_list) => {
            println!("Using StuDocu method");
            let new_contents: Vec<Vec<(u32, u16)>> =
                content_list.iter().skip(1).map(|x| vec![x[1]]).collect();

            let vector: Vec<(&u32, &(u32, u16))> = pages.iter().filter(|x| *x.0 != 1).collect();
            for (i, page) in vector.iter().enumerate() {
                let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                let contents_objects: Vec<Object> = new_contents[i]
                    .iter()
                    .map(|x| Object::Reference(*x))
                    .collect();

                mutable_page.set(*b"Contents", lopdf::Object::Array(contents_objects));

                mutable_page.set("Annots", Object::Array(vec![]));
            }
            (vec![1], 1)
        }

        method::Method::Naive => {
            println!("Using naive method");
            let mut to_delete = Vec::new();

            for page in &pages {
                let page_type =
                    page_type::PageType::get_page_type(&doc, page.1).unwrap_or_default();
                let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();

                let mediabox = mutable_page.get(b"MediaBox").unwrap().as_array().unwrap();
                let height_offset = match mediabox[1].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[1].as_i64().unwrap() as f32,
                };
                let width_offset = match mediabox[0].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[0].as_i64().unwrap() as f32,
                };

                let height = match mediabox[3].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[3].as_i64().unwrap() as f32,
                };
                let width = match mediabox[2].as_f32() {
                    Ok(h) => h,
                    _ => mediabox[2].as_i64().unwrap() as f32,
                };

                match page_type {
                    page_type::PageType::FullPageAds => to_delete.push(*page.0),
                    page_type::PageType::Idk => to_delete.push(*page.0),
                    page_type::PageType::BannerAds => {
                        //1.141
                        let scale = 1.124;
                        for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                            mutable_page.set(
                                _box,
                                Object::Array(vec![
                                    Object::Real(
                                        0.164 * (width - width_offset) + width_offset * scale,
                                    ),
                                    Object::Real(
                                        0.031 * (height - height_offset) + height_offset * scale,
                                    ),
                                    Object::Real(
                                        0.978 * (width - width_offset) * scale
                                            + width_offset * scale,
                                    ),
                                    Object::Real(
                                        0.865 * (height - height_offset) * scale
                                            + height_offset * scale,
                                    ),
                                ]),
                            );
                        }

                        let mut contents = doc.get_page_content(*page.1).unwrap();
                        let mut new_contents = Vec::new();
                        let c_prepend = "q\n1.124 0 0 1.124 0 0 cm\n".as_bytes();
                        let c_append = "Q".as_bytes();

                        new_contents.extend_from_slice(c_prepend);
                        new_contents.append(&mut contents);
                        new_contents.extend_from_slice(c_append);

                        doc.change_page_content(*page.1, new_contents).unwrap()
                    }
                    page_type::PageType::Watermark => {
                        for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                            mutable_page.set(
                                _box,
                                Object::Array(vec![
                                    Object::Real(0.015 * (width - width_offset) + width_offset),
                                    Object::Real(0.05 * (height - height_offset) + height_offset),
                                    Object::Real(0.95 * (width - width_offset) + width_offset),
                                    Object::Real(0.98 * (height - height_offset) + height_offset),
                                ]),
                            );
                        }
                    }
                }
            }

            for page in &pages {
                // remove the logo
                let _ = remove_logo(&mut doc, page.1);

                // remove the annotations
                let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                mutable_page.set("Annots", Object::Array(vec![]));
            }

            (to_delete, 2)
        }
    };

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

            if contents.len() == 1 {
                return true;
            } else {
                return false;
            }
        })
        .map(|x| *x.0)
        .collect();

    if content_list
        .iter()
        .map(|x| x.len())
        .filter(|x| *x == 3)
        .collect::<Vec<_>>()
        .len()
        > 1
    {
        return Method::StuDocu(content_list);
    }

    if content_list.len() > 1
        && content_list[0]
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&content_list[1].iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .len()
            > 1
    {
        return Method::Wuolah(content_list, to_delete);
    }
    Method::Naive
}
