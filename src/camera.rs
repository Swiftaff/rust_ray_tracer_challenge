use std::time::Instant;

use crate::canvas;
use crate::matrices;
use crate::rays;

use crate::tuples;
use crate::worlds;

#[derive(Debug, Clone)]
pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub half_width: f64,
    pub half_height: f64,
    pub field_of_view: f64,
    pub transform: matrices::Matrix4,
    pub pixel_size: f64,
}

impl Camera {
    pub fn ray_for_pixel(&self, px: u32, py: u32) -> rays::Ray {
        //the offset from the edge of the canvas to the pixel's center
        let xoffset: f64 = (px as f64 + 0.5) * self.pixel_size;
        let yoffset: f64 = (py as f64 + 0.5) * self.pixel_size;

        //the untransformed coordinates of the pixel in world space.
        //(remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x: f64 = self.half_width - xoffset;
        let world_y: f64 = self.half_height - yoffset;

        //using the camera matrix, transform the canvas point and the origin,
        //and then compute the ray's direction vector.
        //(remember that the canvas is at z=-1)
        let pixel: tuples::Point = self
            .transform
            .inverse()
            .tuple_multiply(&tuples::point(world_x, world_y, -1.0));
        let origin: tuples::Point = self
            .transform
            .inverse()
            .tuple_multiply(&tuples::point(0.0, 0.0, 0.0));
        let direction: tuples::Vector = pixel.subtract(&origin).normalize();

        rays::ray(origin, direction)
    }

    pub fn render(&self, w: &worlds::World) -> canvas::PixelCanvas {
        let mut image = canvas::pixel_canvas(self.hsize, self.vsize, tuples::COLOR_BLACK);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let col = w.color_at(&r, &worlds::RECURSIVE_DEPTH);
                image = image.pixel_write(&x, &y, col);
            }
        }
        image
    }

    pub fn render_percent_message(&self, w: worlds::World, incr: f64) -> canvas::PixelCanvas {
        let mut image = canvas::pixel_canvas(self.hsize, self.vsize, tuples::COLOR_BLACK);
        let mut pc = 0.0;
        let timer = Instant::now();
        for y in 0..self.vsize {
            pc = percent_message(y as f64, self.vsize as f64, pc, incr, timer.elapsed());
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let col = w.color_at(&r, &worlds::RECURSIVE_DEPTH);
                image = image.pixel_write(&x, &y, col);
            }
        }
        image
    }
}

pub fn camera(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
    let half_view: f64 = (field_of_view / 2.0).tan();
    let aspect: f64 = (hsize as f64) / vsize as f64;
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
        half_width: half_width,
        half_height: half_height,
        field_of_view: field_of_view,
        transform: matrices::IDENTITY_MATRIX,
        pixel_size: pixel_size,
    }
}

pub fn percent_message(
    val: f64,
    total: f64,
    mut pc: f64,
    incr: f64,
    timer: std::time::Duration,
) -> f64 {
    let progress = val as f64 / total as f64;
    if progress > pc {
        let total_time_estimated = timer.as_secs_f64() / pc;
        let remaining_time_estimated = total_time_estimated - progress;
        let remaining_str = if remaining_time_estimated > 60.0 {
            format!("{} mins", remaining_time_estimated / 60.0)
        } else {
            format!("{} seconds", remaining_time_estimated.to_string())
        };
        println!(
            "...ray tracing: {:.0}%. Time so far: {:?}. Expected Remaining: {}",
            pc * 100.0,
            timer,
            remaining_str
        );
        pc = progress + incr;
    }
    pc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations;
    use std::f64::consts::PI;

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
            tuples::get_bool_numbers_are_equal(&(c.field_of_view as f64), &(PI / 2.0)),
            true
        );
        assert_eq!(matrices::IDENTITY_MATRIX.is_equal_to(&c.transform), true);
    }

    #[test]
    fn test_pixel_size_horizontal_canvas() {
        //The pixel size for a horizontal canvas
        let c = camera(200, 150, PI / 2.0);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&c.pixel_size, &0.01),
            true
        );
    }

    #[test]
    fn test_pixel_size_vertical_canvas() {
        //The pixel size for a vertical canvas
        let c = camera(125, 200, PI / 2.0);
        assert_eq!(
            tuples::get_bool_numbers_are_equal(&c.pixel_size, &0.01),
            true
        );
    }

    #[test]
    fn test_constructing_ray_through_center_of_canvas() {
        //Constructing a ray through the center of the canvas
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin.is_equal_to(&tuples::point(0.0, 0.0, 0.0)), true);
        assert_eq!(
            r.direction.is_equal_to(&tuples::vector(0.0, 0.0, -1.0)),
            true
        );
    }

    #[test]
    fn test_constructing_ray_through_corner_of_canvas() {
        //Constructing a ray through a corner of the canvas
        let c = camera(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin.is_equal_to(&tuples::point(0.0, 0.0, 0.0)), true);
        assert_eq!(
            r.direction
                .is_equal_to(&tuples::vector(0.66519, 0.33259, -0.66851)),
            true
        );
    }

    #[test]
    fn test_constructing_ray_when_camera_is_transformed() {
        //Constructing a ray when the camera is transformed
        let mut c = camera(201, 101, PI / 2.0);
        let rot = transformations::matrix4_rotation_y_rad(PI / 4.0);
        let tran = transformations::matrix4_translation(0.0, -2.0, 5.0);
        c.transform = rot.multiply(&tran);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin.is_equal_to(&tuples::point(0.0, 2.0, -5.0)), true);
        assert_eq!(
            r.direction.is_equal_to(&tuples::vector(
                2.0_f64.sqrt() / 2.0,
                0.0,
                -2.0_f64.sqrt() / 2.0
            )),
            true
        );
    }

    #[test]
    fn test_rendering_world_with_camera() {
        //Rendering a world with a camera
        let w = worlds::world_default();
        let from = tuples::point(0.0, 0.0, -5.0);
        let to = tuples::point(0.0, 0.0, 0.0);
        let up = tuples::vector(0.0, 1.0, 0.0);
        let mut c = camera(11, 11, PI / 2.0);
        c.transform = transformations::view_transform(&from, &to, &up);
        let image = c.render(&w);
        let pa = image.get_at(&5, &5);
        let col = tuples::color(0.38066, 0.47583, 0.2855);
        assert_eq!(pa.is_equal_to(&col), true);
    }
}
