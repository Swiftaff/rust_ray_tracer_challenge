use std::f64::consts::PI;

use crate::matrices;
use crate::transformations;
use crate::tuples;

#[derive(Debug, Clone)]
pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
    pub transform: matrices::Matrix4,
    pub pixel_size: f64,
}

pub fn camera(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
    let half_view: f64 = (field_of_view / 2.0).tan();
    let aspect: f64 = (hsize.clone() as f64) / vsize.clone() as f64;
    let mut half_width: f64 = half_view * aspect;
    let mut half_height: f64 = half_view;
    if aspect >= 1.0 {
        half_width = half_view;
        half_height = half_view / aspect;
    }
    let pixel_size = (half_width * 2.0) / hsize as f64;
    Camera {
        hsize: hsize,
        vsize: vsize,
        field_of_view: field_of_view,
        transform: matrices::IDENTITY_MATRIX,
        pixel_size: pixel_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructing_a_camera() {
        //Constructing a camera
        let hsize: u32 = 160;
        let vsize: u32 = 120;
        let field_of_view: f64 = PI / 2.0;
        let c = camera(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(c.field_of_view as f64, PI / 2.0),
            true
        );
        assert_eq!(
            matrices::get_bool_equal_m4(matrices::IDENTITY_MATRIX, c.transform),
            true
        );
    }

    #[test]
    fn test_pixel_size_horizontal_canvas() {
        //The pixel size for a horizontal canvas
        let c = camera(200, 150, PI / 2.0);
        assert_eq!(tuples::get_bool_numbers_are_equal(c.pixel_size, 0.01), true);
    }

    #[test]
    fn test_pixel_size_vertical_canvas() {
        //The pixel size for a vertical canvas
        let c = camera(125, 200, PI / 2.0);
        assert_eq!(tuples::get_bool_numbers_are_equal(c.pixel_size, 0.01), true);
    }
}
