//! The `image_tools` module contains various utility functions for 
//! image manipulation.

extern crate image;

use image::GenericImageView;
//use core::fmt::Display;
use SplitMode::*;

/// The `SplitMode` enum contains all possible modes of splitting a image into 
/// subimages of a defined size.
//#[non_exhaustive]
//#[derive(Debug)]
pub enum SplitMode {
    /// A mode to producing overlapping sub images at the left and bottom edges 
    /// if there is no way of perfectly splitting the image.
    EdgeOverlapLeftBottomMode,
    /// A custom splitting mode.
    CustomMode(Box<dyn Fn(u32,u32,u32,u32) -> Vec<(u32, u32)>>),
}

impl SplitMode {
    fn get_starts(&self, image_width: u32, image_height: u32, split_width: u32, split_height: u32) -> Vec<(u32, u32)> {
        match self {
            EdgeOverlapLeftBottomMode => combine_height_width(
                    &split_range_perfect(image_height, split_height),
                    &split_range_perfect(image_width, split_width)
                ),
            CustomMode(custom_function) => custom_function(image_width, image_height, split_width, split_height),
        }
    }
}

impl Default for SplitMode {
    fn default() -> Self { EdgeOverlapLeftBottomMode }
}

pub trait SplitableImageExt where Self : Sized {
    fn split_into(&mut self, height: u32, width: u32, mode: SplitMode) -> Vec<Self>;
}

impl SplitableImageExt for image::DynamicImage {
    fn split_into(&mut self, height: u32, width: u32, mode: SplitMode) -> Vec<Self> {
        if height > 0 && width > 0 {
            // Only split images if the image can be split.
            if self.height() >= height && self.width() >= width {
                mode.get_starts(self.width(), self.height(), width, height).iter()
                    .map(|start| self.crop(start.1, start.0, width, height))
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            panic!("Cannot split into images of 0 pixels.")
        }
    }
}

fn split_range_perfect(original: u32, split: u32) -> Vec<u32> {
    if split == 0 {
        panic!("Splitting into 0 is not possible.");
    } else if original < split {
        return Vec::new();
    }
    let mut range: Vec<u32> = (0..(original / split))
        .map(|h| h * split)
        .collect();
    if original % split != 0 {
        range.push(original - split);
    }
    range
}

fn combine_height_width(heights: &[u32], widths: &[u32]) -> Vec<(u32, u32)> {
    heights.iter()
        .flat_map(|h| widths.iter().map(move |w| (*h, *w)))
        .collect()
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_split_range_perfect() {
        // Test zero input length.
        assert_eq!(split_range_perfect(0, 12), Vec::<u32>::new());
        // Test input length smaller than split length.
        assert_eq!(split_range_perfect(364, 2490), Vec::<u32>::new());
        // Test normal behaviour without overlap.
        assert_eq!(split_range_perfect(50000, 10000), vec!(0,10000,20000,30000,40000));
        // Test normal behaviour with overlap.
        assert_eq!(split_range_perfect(50067, 10000), vec!(0,10000,20000,30000,40000,40067));
    }
    
    #[test]
    #[should_panic(expected="Splitting into 0 is not possible.")]
    fn test_split_range_perfect_panic_zero() {
        split_range_perfect(43, 0);
    }
    
    #[test]
    fn test_combine_height_width() {
        let heights = vec!(7, 24987, 78);
        let widths = vec!(12, 943, 44944);
        // Test empty height input.
        assert_eq!(combine_height_width(&Vec::new(), &widths), Vec::<(u32, u32)>::new());
        // Test empty width input.
        assert_eq!(combine_height_width(&heights, &Vec::new()), Vec::<(u32, u32)>::new());
        let combined_assertion = vec!(
            (7, 12), (7, 943), (7, 44944),
            (24987, 12), (24987, 943), (24987, 44944),
            (78, 12), (78, 943), (78, 44944)
        );
        // Check if every element is present without caring for the order of elements.
        let combined_result = combine_height_width(&heights, &widths);
        assert_eq!(combined_assertion.len(), combined_result.len());
        for assertion in combined_assertion {
            assert!(combined_result.contains(&assertion));
        }
    }
}

