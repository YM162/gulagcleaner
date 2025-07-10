use std::{collections::HashSet, error::Error, vec};

use lopdf::{Dictionary, Document, Object, ObjectId};

use crate::{clean::Cleaner, models::page_type};

#[derive(Debug)]
pub enum Method {
    /// The Wuolah method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts,
    /// and a vector of unsigned integers as parameters.
    Wuolah(Vec<Vec<(u32, u16)>>, Vec<u32>),
    /// The StuDocu method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts
    /// as a parameter.
    StuDocu(Vec<Vec<(u32, u16)>>),
    /// The Naive method, which does not take any parameters.
    Naive,
}

/// Implements the `Cleaner` trait for the `Method` enum.
/// This method cleans the document based on the selected method.
/// It modifies the contents and properties of the document's pages.
/// Returns a tuple containing the pages to delete and a status code.
impl Cleaner for Method {
    fn clean(&mut self, doc: &mut Document) -> (Vec<u32>, u8) {
        println!("Cleaning with method: {self:?}");
        match self {
            Method::Wuolah(content_list, to_delete) => {
                let new_contents: Vec<Vec<(u32, u16)>> = content_list
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let pares = if x == content_list.last().unwrap() {
                            find_iobj_pairs(x, &content_list[i - 1])
                        } else {
                            let check_if_00 = find_iobj_pairs(x, &content_list[i + 1]);
                            if check_if_00 == (0, 0) {
                                let check_again_if_00 = find_iobj_pairs(x, &content_list[i - 1]);
                                if check_again_if_00 == (0, 0) {
                                    (0, 0)
                                } else {
                                    check_again_if_00
                                }
                            } else {
                                check_if_00
                            }
                        };

                        x[(pares.0) - 2..=(pares.1) + 3].to_vec()
                    })
                    .collect();

                let pages = doc.get_pages();

                let vector: Vec<(&u32, &(u32, u16))> = pages
                    .iter()
                    .filter(|x| doc.get_page_contents(*x.1).len() > 3)
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

                (to_delete.to_vec(), 0)
            }
            Method::StuDocu(content_list) => {
                let new_contents: Vec<Vec<(u32, u16)>> =
                    content_list.iter().skip(1).map(|x| vec![x[1]]).collect();
                let pages = doc.get_pages();
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

            Method::Naive => {
                println!("Using naive method");
                let mut to_delete = Vec::new();
                let pages = doc.get_pages();
                for page in &pages {
                    let page_type =
                        page_type::PageType::get_page_type(doc, page.1).unwrap_or_default();
                        println!("{page_type:?}");
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
                                            0.031 * (height - height_offset)
                                                + height_offset * scale,
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
                                        Object::Real(
                                            0.05 * (height - height_offset) + height_offset,
                                        ),
                                        Object::Real(0.95 * (width - width_offset) + width_offset),
                                        Object::Real(
                                            0.98 * (height - height_offset) + height_offset,
                                        ),
                                    ]),
                                );
                            }
                        }
                    }
                }

                for page in &pages {
                    // remove the logo
                    let _ = remove_logo(doc, page.1);

                    // remove the annotations
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                    mutable_page.set("Annots", Object::Array(vec![]));
                }

                (to_delete, 2)
            }
        }
    }
}

pub fn find_iobj_pairs(first_page: &[(u32, u16)], second_page: &[(u32, u16)]) -> (usize, usize) {
    let unique_first_page: HashSet<&(u32, u16)> = first_page.iter().collect();
    let unique_second_page: HashSet<&(u32, u16)> = second_page.iter().collect();

    let c: Vec<&&(u32, u16)> = unique_first_page
        .intersection(&unique_second_page)
        .collect();
    println!("{c:?}");

    //It seems like the indexes are always c.len() - 3 and c.len() - 2, except for the len == 2 case.

    let mut indexes: Vec<usize> = c
        .iter()
        .map(|&&element| first_page.iter().position(|&r| r == *element).unwrap())
        .collect();
    
    indexes.sort();

    let len = indexes.len();
    if len == 2 {
        return (indexes[0], indexes[1]);
    }
    if len < 2 {
        return (0, 0);
    }

    (indexes[len - 3], indexes[len - 2])
}

pub fn remove_logo(doc: &mut Document, page: &ObjectId) -> Result<(), Box<dyn Error>> {
    let xobjs = get_xobjs(doc, page)?.clone();
    let images = get_images(doc, &xobjs)?;

    // let has_logo = !page_type::LOGO_DIMS
    //     .iter()
    //     .collect::<HashSet<_>>()
    //     .intersection(&images.iter().collect::<HashSet<_>>())
    //     .collect::<Vec<_>>()
    //     .is_empty();

    let mut has_logo = images
        .iter()
        .any(|image| page_type::LOGO_DIMS.contains(image));
    
    //See if there are two images with the same dimensions
    let mut image_dims = HashSet::new();
    let mut repeated_logo_dims = (0, 0);
    for image in images {
        if !image_dims.insert(image) {

            if image.1 == 0 || image.0 == 0 {
                continue;
            }

            if  (image.1 as f64 / image.0 as f64) > 5_f64 &&  (image.1 as f64) / (image.0 as f64) < 6_f64 {
                repeated_logo_dims = image;
                has_logo = true;
                break;
            }
        }
    }

    println!("{repeated_logo_dims:?}");


    if !has_logo {
        return Ok(());
    }
    for obj in &xobjs {
        let objectdict = get_objdict(doc, obj)?;

        let subtype = objectdict.get(b"Subtype")?.as_name()?;

        let sub_s = String::from_utf8_lossy(subtype);
        if sub_s.starts_with("Image")
            && (page_type::LOGO_DIMS.contains(&(
                objectdict.get(b"Height")?.as_i64()?,
                objectdict.get(b"Width")?.as_i64()?,
            )) || repeated_logo_dims == (
                objectdict.get(b"Height")?.as_i64()?,
                objectdict.get(b"Width")?.as_i64()?,
            )
        )
        {
            let mutable_page = &mut doc
                .get_object_mut(obj.1.as_reference()?)?
                .as_stream_mut()?
                .dict;
            mutable_page.set(*b"Height", 0);
        }
    }
    Ok(())
}

fn get_objdict<'a>(
    doc: &'a Document,
    obj: (&Vec<u8>, &Object),
) -> Result<&'a Dictionary, Box<dyn Error>> {
    let objdict = &doc
        .get_object(obj.1.as_reference().unwrap())?
        .as_stream()?
        .dict;

    Ok(objdict)
}

pub fn get_xobjs<'a>(doc: &'a Document, page: &ObjectId) -> Result<&'a Dictionary, Box<dyn Error>> {
    let resource = doc.get_page_resources(*page)?;
    let resource_dict: &Dictionary = if resource.1.is_empty() {
        resource.0.unwrap()
    } else {
        doc.get_object(resource.1[0])?.as_dict()?
    };
    let xobjs = match resource_dict.get(b"XObject")? {
        Object::Dictionary(x) => x,
        Object::Reference(x) => doc.get_object(*x)?.as_dict()?,
        _ => return Err("Error".into()),
    };
    Ok(xobjs)
}

pub fn get_images(doc: &Document, xobjs: &Dictionary) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let mut images = Vec::new();

    for obj in xobjs {
        let objectdict = get_objdict(doc, obj)?;

        let subtype = objectdict.get(b"Subtype").unwrap().as_name().unwrap();
        let sub_s = String::from_utf8_lossy(subtype);

        if sub_s.starts_with("Image") {
            images.push((
                objectdict.get(b"Height").unwrap().as_i64().unwrap(),
                objectdict.get(b"Width").unwrap().as_i64().unwrap(),
            ));
        }
    }

    Ok(images)
}
