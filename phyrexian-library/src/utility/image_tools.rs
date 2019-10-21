//! The `image_tools` module contains various utility functions for 
//! image manipulation.

extern crate image;

use std::num::NonZeroU32;
use core::borrow::Borrow;
use image::GenericImageView;
use core::fmt::Display;
use SplitMode::*;

#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
/// An point / pixel coordinate on an image.
pub struct ImagePoint {
    /// The pixel on the x-axis corresponding to the width.
    x: u32,
    /// The pixel on the y-axis corresponding to the height.
    y: u32,
}

impl ImagePoint {
    /// Creates an `ImagePoint` from a x- and y-coordinate.
    pub fn new<P>(x: P, y: P) -> Self where P: Borrow<u32>{
        ImagePoint{x: *x.borrow(), y: *y.borrow()}
    }
    
    /// Returns the x-coordinate of this point.
    pub fn x(self) -> u32 {
        self.x
    }
    
    /// Returns the y-coordinate of this point.
    pub fn y(self) -> u32 {
        self.y
    }
}

impl<P> From<&(P, P)> for ImagePoint where P: Borrow<u32> {
    fn from(point: &(P, P)) -> Self {
        ImagePoint::new(point.0.borrow(), point.1.borrow())
    }
}

impl<P> From<(P, P)> for ImagePoint where P: Borrow<u32> {
    fn from(point: (P, P)) -> Self {
        ImagePoint::new(point.0, point.1)
    }
}

impl Into<(u32, u32)> for ImagePoint {
    fn into(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl Display for ImagePoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(x = {} , y = {})", self.x, self.y)
    }
}

/// The `SplitMode` enum contains all possible modes of splitting a image into 
/// subimages of a defined size.
//#[non_exhaustive]
//#[derive(Debug)]
pub enum SplitMode {
    /// A mode to producing overlapping sub images at the left and bottom edges 
    /// if there is no way of perfectly splitting the image.
    EdgeOverlapBottomLeftMode,
    /// A custom splitting mode.
    CustomMode(Box<dyn Fn(u32, u32, NonZeroU32, NonZeroU32) -> Vec<ImagePoint>>),
}

impl SplitMode {
    fn get_starts(&self, image_width: u32, image_height: u32, split_width: NonZeroU32, split_height: NonZeroU32) -> Vec<ImagePoint> {
        match self {
            EdgeOverlapBottomLeftMode => combine_coordinates(
                &split_range_align_end(image_width, split_width),
                &split_range_align_end(image_height, split_height)
            ),
            CustomMode(custom_function) => custom_function(image_width, image_height, split_width, split_height),
        }
    }
}

impl Default for SplitMode {
    fn default() -> Self { EdgeOverlapBottomLeftMode }
}

pub trait SplitableImageExt where Self : Sized {
    fn split_into(&mut self, height: NonZeroU32, width: NonZeroU32, mode: SplitMode) -> Vec<Self>;
}

impl SplitableImageExt for image::DynamicImage {
    fn split_into(&mut self, height: NonZeroU32, width: NonZeroU32, mode: SplitMode) -> Vec<Self> {
        let (width_u, height_u) = (width.get(), height.get());
        // Only split images if the image can be split.
        if self.height() >= height_u && self.width() >= width_u {
            mode.get_starts(self.width(), self.height(), width, height).iter()
                .map(|start| self.crop(start.x(), start.y(), width_u, height_u))
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Splits the specified range into parts of the defined length.
/// Overlapping may happen at the end of the range.
/// 
/// If the defined `split` length is longer than the `original` range an
/// empty vector is returned.
/// 
/// # Arguments
/// 
/// * `original` - A number representing a continous range.
/// * `split` - The length of the parts to split the specified range into. 
/// this cannot be zero.
fn split_range_align_end(original: u32, split: NonZeroU32) -> Vec<u32> {
    let split = split.get();
    if original < split {
        Vec::new()
    } else {
        let mut range: Vec<u32> = (0..(original / split))
            .map(|h| h * split)
            .collect();
        if original % split != 0 {
            range.push(original - split);
        }
        range
    }

}

/// Combines the coordinates into [`ImagePoint`]s by forming every 
/// possible x-y-pair.
/// 
/// # Arguments
/// 
/// * `x_coordinates` - A list of x-coordinates.
/// * `y_coordinates` - A list of y-coordinates.
/// 
/// [`ImagePoint`]: ./struct.ImagePoint.html
fn combine_coordinates(x_coordinates: &[u32], y_coordinates: &[u32]) -> Vec<ImagePoint> {
    x_coordinates.iter()
        .flat_map(|x| y_coordinates.iter().map(move |y| ImagePoint::new(x, y)))
        .collect()
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test_split_align_end() {
        // Test zero input length.
        assert_eq!(split_range_align_end(0, NonZeroU32::new(12).unwrap()), Vec::<u32>::new());
        // Test input length smaller than split length.
        assert_eq!(split_range_align_end(364, NonZeroU32::new(2490).unwrap()), Vec::<u32>::new());
        // Test normal behaviour without overlap.
        assert_eq!(split_range_align_end(50000, NonZeroU32::new(10000).unwrap()), vec!(0,10000,20000,30000,40000));
        // Test normal behaviour with overlap.
        assert_eq!(split_range_align_end(50067, NonZeroU32::new(10000).unwrap()), vec!(0,10000,20000,30000,40000,40067));
    }
    
    #[test]
    fn test_combine_coordinates() {
        let x = vec!(7, 24987, 78);
        let y = vec!(12, 943, 44944);
        // Test empty height input.
        assert_eq!(combine_coordinates(&Vec::new(), &y), Vec::<ImagePoint>::new());
        // Test empty width input.
        assert_eq!(combine_coordinates(&x, &Vec::new()), Vec::<ImagePoint>::new());
        let combined_assertion = [
            (7, 12), (7, 943), (7, 44944),
            (24987, 12), (24987, 943), (24987, 44944),
            (78, 12), (78, 943), (78, 44944)
        ].iter().map(ImagePoint::from);
        // Check if every element is present without caring for the order of elements.
        let combined_result = combine_coordinates(&x, &y);
        assert_eq!(combined_assertion.len(), combined_result.len());
        for assertion in combined_assertion {
            assert!(combined_result.contains(&assertion));
        }
    }
}

