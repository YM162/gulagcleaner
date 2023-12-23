use std::{error::Error, collections::HashSet};

use lopdf::{Document, ObjectId};

use crate::{get_xobjs, get_images};

#[derive(Default)]
pub enum PageType {
    BannerAds,
    FullPageAds,
    Watermark,
    #[default]
    Idk,
}

pub const LOGO_DIMS: [(i64, i64); 3] = [(71, 390), (37, 203), (73, 390)];

const HORIZONTAL_BANNER_DIMS: [(i64, i64); 7] = [
    (247, 1414),
    (213, 1219),
    (215, 1219),
    (249, 1414),
    (217, 1240),
    (147, 1757),
    (221, 1240)
];
const VERTICAL_BANNER_DIMS: [(i64, i64); 8] = [
    (1753, 170),
    (1518, 248),
    (1520, 147),
    (1753, 177),
    (1751, 171),
    (1537, 147),
    (1093, 217),
    (1534, 150)
];
const FULL_PAGE_DIMS: [(i64, i64); 7] = [
    (842, 595),
    (1754, 1240),
    (2526, 1785),
    (1733, 1219),
    (3508, 2480),
    (2339, 1653),
    (1785,2526),
];

impl PageType {
    pub fn get_page_type(doc: &Document, page: &ObjectId) -> Result<PageType, Box<dyn Error>> {
        let xobjs = get_xobjs(doc, page)?;
        let images = get_images(doc, xobjs)?;
        println!("Images: {:?}",images);
        let has_logo = !LOGO_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        let has_horizontal_banner = !HORIZONTAL_BANNER_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        let has_vertical_banner = !VERTICAL_BANNER_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        let has_full_page = !FULL_PAGE_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        if has_horizontal_banner && has_vertical_banner {
            Ok(PageType::BannerAds)
        } else if has_full_page {
            Ok(PageType::FullPageAds)
        } else if has_logo {
            Ok(PageType::Watermark)
        } else {
            Ok(PageType::Idk)
        }
    }
}