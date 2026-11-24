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
                // Content streams that draw the original page, for every page. Each page is
                // analysed on its own first, so this works whatever its orientation and even
                // if the document only has one page with ads. Comparing it with its neighbours
                // is kept as a fallback. `None` means an unknown structure: the page is left
                // untouched.
                let new_contents: Vec<Option<Vec<(u32, u16)>>> = content_list
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        find_original_contents(doc, x)
                            .or_else(|| find_shared_contents(content_list, i))
                            .map(|(first, last)| x[first..=last].to_vec())
                    })
                    .collect();

                let pages = doc.get_pages();

                let vector: Vec<(&u32, &(u32, u16))> = pages
                    .iter()
                    .filter(|x| doc.get_page_contents(*x.1).len() > 3)
                    .collect();
                for (i, page) in vector.iter().enumerate() {
                    let Some(page_contents) = &new_contents[i] else {
                        continue;
                    };
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                    let contents_objects: Vec<Object> = page_contents
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

/// Locates the original contents of page `i` by looking at the content streams it shares
/// with the next or the previous page (the method used before [`find_original_contents`]).
///
/// Returns the inclusive range of indexes into `content_list[i]`, or `None` if no
/// neighbouring page shares content streams with it.
fn find_shared_contents(content_list: &[Vec<(u32, u16)>], i: usize) -> Option<(usize, usize)> {
    let page = &content_list[i];
    let (first, last) = [Some(i + 1), i.checked_sub(1)]
        .into_iter()
        .flatten()
        .filter_map(|j| content_list.get(j))
        .map(|other| find_iobj_pairs(page, other))
        .find(|&pairs| pairs != (0, 0))?;

    let first = first.checked_sub(2)?;
    let last = last + 3;
    (last < page.len()).then_some((first, last))
}

/// `q [cm]` streams at the start of a Wuolah page that belong to the original page (the plain
/// `q` plus the scale and translation that fit it to the page size) and therefore must be kept.
const ORIGINAL_PAGE_WRAPPERS: usize = 3;

/// Finds the content streams that draw the original page in a page made by Wuolah.
///
/// Wuolah builds its pages with pdf-lib: it wraps the original contents in `q <matrix> cm`
/// streams that shrink and move them to make room for the ads, and then appends the ads
/// (banners, logo, footer, hidden text) as more content streams:
///
/// ```text
/// q a b c d e f cm   ┐ ad layout (removed)
/// ...                ┘
/// q a b c d e f cm   ┐
/// q a b c d e f cm   │ original page (kept)
/// q                  │
/// <original page>    │
/// Q                  │
/// Q                  │
/// Q                  ┘
/// <ads>              ┐ (removed)
/// Q ...              ┘
/// ```
///
/// The original page is found by following the graphics state nesting (`q`/`Q`) of the page
/// itself, so it does not depend on the page orientation, on the size of the ads, nor on
/// comparing the page with other pages.
///
/// Returns the inclusive range of indexes into `contents`, or `None` if the page does not
/// have this structure.
pub fn find_original_contents(doc: &Document, contents: &[ObjectId]) -> Option<(usize, usize)> {
    let stream_data = |id: &ObjectId| {
        doc.get_object(*id)
            .and_then(Object::as_stream)
            .and_then(|stream| stream.get_plain_content())
            .unwrap_or_default()
    };
    let mut streams = contents.iter().map(stream_data).enumerate().peekable();

    let mut wrappers: usize = 0;
    while streams
        .next_if(|(_, data)| is_wrapper_stream(data))
        .is_some()
    {
        wrappers += 1;
    }
    let first = wrappers.checked_sub(ORIGINAL_PAGE_WRAPPERS)?;

    // Follow the nesting until the outermost wrapper of the original page is closed.
    let mut depth = ORIGINAL_PAGE_WRAPPERS as i64;
    for (i, data) in streams {
        let (net, lowest) = graphics_state_balance(&data);
        if depth + lowest > 0 {
            depth += net;
            continue;
        }
        // It must be closed by one of the `Q` streams written by pdf-lib, and there must be
        // something left to remove: otherwise this is not a Wuolah page.
        let removes_something = first > 0 || i + 1 < contents.len();
        return (depth == 1 && is_pop_stream(&data) && removes_something).then_some((first, i));
    }
    None
}

/// Whether a content stream only saves the graphics state and, optionally, applies a
/// transformation matrix (`q` or `q a b c d e f cm`), like the ones written by pdf-lib.
fn is_wrapper_stream(content: &[u8]) -> bool {
    let tokens: Vec<&[u8]> = tokens(content).take(9).collect();
    match tokens.as_slice() {
        [b"q"] => true,
        [b"q", matrix @ .., b"cm"] => matrix.len() == 6 && matrix.iter().all(|t| is_number(t)),
        _ => false,
    }
}

/// Whether a content stream only restores the graphics state (`Q`).
fn is_pop_stream(content: &[u8]) -> bool {
    let mut tokens = tokens(content);
    tokens.next() == Some(&b"Q"[..]) && tokens.next().is_none()
}

fn tokens(content: &[u8]) -> impl Iterator<Item = &[u8]> + '_ {
    content
        .split(|&c| is_pdf_whitespace(c))
        .filter(|token| !token.is_empty())
}

fn is_number(token: &[u8]) -> bool {
    token.iter().any(u8::is_ascii_digit)
        && token
            .iter()
            .all(|c| c.is_ascii_digit() || matches!(c, b'+' | b'-' | b'.'))
}

/// Effect of a content stream on the graphics state stack, as `(net, lowest)`: `net` is the
/// number of `q` minus the number of `Q` operators, and `lowest` the lowest depth reached,
/// relative to the start of the stream (so it is never positive).
///
/// Strings, hex strings, comments, names and inline image data are skipped, so that their
/// contents are not mistaken for operators.
pub(crate) fn graphics_state_balance(content: &[u8]) -> (i64, i64) {
    let (mut depth, mut lowest) = (0_i64, 0_i64);
    let mut i = 0;
    while i < content.len() {
        i = match content[i] {
            c if is_pdf_whitespace(c) => i + 1,
            b'%' => content[i..]
                .iter()
                .position(|&c| c == b'\n' || c == b'\r')
                .map_or(content.len(), |end| i + end),
            b'(' => skip_literal_string(content, i),
            b'<' if content.get(i + 1) == Some(&b'<') => i + 2,
            b'<' => content[i..]
                .iter()
                .position(|&c| c == b'>')
                .map_or(content.len(), |end| i + end + 1),
            b'/' => end_of_token(content, i + 1),
            c if is_pdf_delimiter(c) => i + 1,
            _ => {
                let end = end_of_token(content, i);
                match &content[i..end] {
                    b"q" => depth += 1,
                    b"Q" => {
                        depth -= 1;
                        lowest = lowest.min(depth);
                    }
                    _ => {}
                }
                if &content[i..end] == b"ID" {
                    skip_inline_image_data(content, end)
                } else {
                    end
                }
            }
        };
    }
    (depth, lowest)
}

fn is_pdf_whitespace(c: u8) -> bool {
    matches!(c, b'\0' | b'\t' | b'\n' | b'\x0C' | b'\r' | b' ')
}

fn is_pdf_delimiter(c: u8) -> bool {
    matches!(
        c,
        b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%'
    )
}

fn end_of_token(content: &[u8], start: usize) -> usize {
    content[start..]
        .iter()
        .position(|&c| is_pdf_whitespace(c) || is_pdf_delimiter(c))
        .map_or(content.len(), |end| start + end)
}

/// Skips a literal string, which can contain balanced and escaped parentheses.
fn skip_literal_string(content: &[u8], start: usize) -> usize {
    let mut nesting = 0;
    let mut i = start;
    while i < content.len() {
        match content[i] {
            b'\\' => i += 1,
            b'(' => nesting += 1,
            b')' => {
                nesting -= 1;
                if nesting == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    content.len()
}

/// Skips the binary data of an inline image, from the end of the `ID` operator to `EI`.
fn skip_inline_image_data(content: &[u8], start: usize) -> usize {
    (start + 1..content.len().saturating_sub(1))
        .find(|&i| {
            content[i] == b'E'
                && content[i + 1] == b'I'
                && is_pdf_whitespace(content[i - 1])
                && content.get(i + 2).is_none_or(|&c| is_pdf_whitespace(c))
        })
        .map_or(content.len(), |i| i + 2)
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
