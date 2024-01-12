use std::{collections::HashSet, error::Error};

use lopdf::{Document, ObjectId};

use super::method::{get_images, get_xobjs};

#[derive(Default)]
/// Represents the different methods used in the Gulag Cleaner application.
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
    (221, 1240),
];
const VERTICAL_BANNER_DIMS: [(i64, i64); 8] = [
    (1753, 170),
    (1518, 248),
    (1520, 147),
    (1753, 177),
    (1751, 171),
    (1537, 147),
    (1093, 217),
    (1534, 150),
];
const FULL_PAGE_DIMS: [(i64, i64); 7] = [
    (842, 595),
    (1754, 1240),
    (2526, 1785),
    (1733, 1219),
    (3508, 2480),
    (2339, 1653),
    (1785, 2526),
];

impl PageType {
    /// Get the type of a page based on its content.
    ///
    /// This function takes a document and a page ID as input and returns the type of the page.
    /// The page type is determined by analyzing the images present in the page.
    /// It checks for the presence of specific image dimensions to identify different types of pages,
    /// such as banner ads, full-page ads, watermarks, or unknown types.
    ///
    /// # Arguments
    ///
    /// * `doc` - A reference to the document containing the page.
    /// * `page` - A reference to the ID of the page.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `PageType` of the page if successful, or a `Box<dyn Error>` if an error occurs.
    pub fn get_page_type(doc: &Document, page: &ObjectId) -> Result<PageType, Box<dyn Error>> {
        let xobjs = get_xobjs(doc, page)?;
        let images = get_images(doc, xobjs)?;
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
