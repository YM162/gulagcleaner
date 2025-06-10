use std::{collections::HashSet, error::Error};
 
use lopdf::{Document, ObjectId};
 
use super::method::{get_images, get_xobjs};
 
#[derive(Default, Debug)]
/// Represents the different methods used in the Gulag Cleaner application.
pub enum PageType {
    BannerAds,
    FullPageAds,
    Watermark,
    #[default]
    Idk,
}
 
pub const LOGO_DIMS: [(i64, i64); 9] = [(71, 390), (37, 203), (73, 390), (23, 130), (24, 130), (19, 109), (20, 109), (72, 391), (24, 129)];
 
const HORIZONTAL_BANNER_DIMS: [(i64, i64); 13] = [
    (247, 1414),
    (213, 1219),
    (215, 1219),
    (249, 1414),
    (217, 1240),
    (147, 1757),
    (148, 1769),
    (221, 1240),
    (136, 780),
    (137,780),
    (218,1241),
    (218,1246),
    (217,1094)
];
const VERTICAL_BANNER_DIMS: [(i64, i64); 14] = [
    (1753, 170),
    (1518, 248),
    (1520, 147),
    (1753, 177),
    (1751, 171),
    (1537, 147),
    (1093, 217),
    (1094, 217),
    (1534, 150),
    (970, 92),
    (969,93),
    (1538, 148),
    (1538, 147),
    (1769,148)
];
const FULL_PAGE_DIMS: [(i64, i64); 10] = [
    (842, 595),
    (1754, 1240),
    (2526, 1785),
    (1733, 1219),
    (3508, 2480),
    (2339, 1653),
    (1785, 2526),
    (1109, 782),
    (1109, 784),
    (1759, 1241),
];
 
/// Check if dimension matches any dimension from a list within a tolerance of 10 units.
fn matches_with_tolerance(dims: &[(i64, i64)], images: &HashSet<(i64, i64)>) -> bool {
    dims.iter().any(|&(w, h)| {
        images.iter().any(|&(iw, ih)| {
            (iw >= w - 10 && iw <= w + 10) && (ih >= h - 10 && ih <= h + 10)
        })
    })
}

/// Generate combined dimensions where two images share either width or height
fn generate_combined_dims(images: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut combined = HashSet::new();
    let img_vec: Vec<_> = images.iter().collect();
    for i in 0..img_vec.len() {
        for j in (i + 1)..img_vec.len() {
            let &(w1, h1) = img_vec[i];
            let &(w2, h2) = img_vec[j];
            if w1 == w2 {
                combined.insert((w1, h1 + h2));
            } else if h1 == h2 {
                combined.insert((w1 + w2, h1));
            }
        }
    }
    combined
}

impl PageType {
    /// Get the type of a page based on its content.
    pub fn get_page_type(doc: &Document, page: &ObjectId) -> Result<PageType, Box<dyn Error>> {
        let xobjs = get_xobjs(doc, page)?;
        let images = get_images(doc, xobjs)?;
        let image_set: HashSet<(i64, i64)> = images.into_iter().collect();
        fn scaled_image_set(images: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
            images.iter().map(|&(w, h)| (w, h * 2)).collect()
        }
        let scaled_images = scaled_image_set(&image_set);
        let combined_dims = generate_combined_dims(&image_set);

        // we compare against combined, scaled and unscaled sets to ensure backwards compatiblity
        let has_horizontal_banner = matches_with_tolerance(&HORIZONTAL_BANNER_DIMS, &combined_dims) ||
                                    matches_with_tolerance(&HORIZONTAL_BANNER_DIMS, &image_set);
                                    matches_with_tolerance(&HORIZONTAL_BANNER_DIMS, &scaled_images);

        let has_vertical_banner = matches_with_tolerance(&VERTICAL_BANNER_DIMS, &combined_dims) ||
                                  matches_with_tolerance(&VERTICAL_BANNER_DIMS, &image_set);
                                  matches_with_tolerance(&VERTICAL_BANNER_DIMS, &scaled_images);

        let has_full_page = matches_with_tolerance(&FULL_PAGE_DIMS, &image_set);
 
        if has_horizontal_banner && has_vertical_banner {
            Ok(PageType::BannerAds)
        } else if has_full_page {
            Ok(PageType::FullPageAds)
        } else {
            let annots = doc.get_page_annotations(*page)?;
 
            let wuolah_annot = annots
                .iter()
                .filter(|x| is_annots_wuolah(x, doc))
                .filter(|x| {
                    x.get(b"Rect").unwrap().as_array().unwrap()[0] == lopdf::Object::Integer(0)
                        || x.get(b"Rect").unwrap().as_array().unwrap()[0] == lopdf::Object::Real(0.0)
                });
            //let mut bannercounter = 0;
            let mut hasfooter = false;
            for annot in wuolah_annot {
                if let Ok(action) = annot.get(b"A") {
                    if let Ok(action_dict) = doc.dereference(action) {
                        if let Ok(uri) = action_dict.1.as_dict().unwrap().get(b"URI") {
                            if let Ok(url) = doc.dereference(uri) {
                                let url_str = url.1.as_string().unwrap();
                                /*if url_str.contains("adU=2") {
                                    bannercounter += 1;
                                }*/
                                if url_str.contains("adU=2") {
                                    hasfooter = true;
                                }
                            }
                        }
                    }
                }
            }
            /*if bannercounter == 1 {
                return Ok(PageType::Watermark);
            }
            if bannercounter > 1 {
                return Ok(PageType::BannerAds);
            }*/
            if hasfooter {
                return Ok(PageType::Watermark);
            }
            Ok(PageType::Idk)
        }
    }
}
 
fn is_annots_wuolah(annot: &&&lopdf::Dictionary, doc: &lopdf::Document) -> bool {
    match annot.get(b"A") {
        Ok(x) => match doc.dereference(x).unwrap().1.as_dict().unwrap().get(b"URI") {
            Ok(y) => {
                let url = doc.dereference(y).unwrap().1.as_string().unwrap();
                if url.contains("track.wlh.es") {
                    !(url.contains("apuntes"))
                } else {
                    false
                }
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}