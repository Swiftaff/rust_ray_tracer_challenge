extern crate image;

use crate::tuples;

const CLAMP_LIMIT: u32 = 255;

#[derive(Debug, Clone)]
pub struct PixelCanvas {
    pub width: u32,
    pub height: u32,
    pub length: u32,
    pub data: Vec<tuples::Color>,
}

pub fn pixel_canvas(width: u32, height: u32, default_color: tuples::Color) -> PixelCanvas {
    let length = width * height;
    let mut data = Vec::with_capacity(length as usize);
    for _i in 0..length {
        data.push(default_color);
    }
    PixelCanvas {
        data: data,
        width: width,
        height: height,
        length: length,
    }
}

pub fn pixel_write(canvas: PixelCanvas, x: u32, y: u32, col: tuples::Color) -> PixelCanvas {
    let index = (canvas.width * y + x) as u32;
    let mut new_canvas = canvas;
    if index < new_canvas.length {
        new_canvas.data[index as usize] = col;
        new_canvas
    } else {
        new_canvas
    }
}

pub fn pixel_get(canvas: PixelCanvas, x: u32, y: u32) -> tuples::Color {
    let index = canvas.width * y + x;
    let mut col = tuples::color(1.0, 0.8, 0.8); //default bright pink?
    if index < canvas.length {
        col = canvas.data[index as usize];
    }
    col
}

pub fn ppm_get(c: PixelCanvas) -> String {
    let header = String::from("P3\n");
    let w = c.width.to_string();
    let h = c.height.to_string();
    let limit = format!("\n{}\n", CLAMP_LIMIT);
    let data = str_from_canvas_data_get(c);
    format!("{}{} {}{}{}", header, w, h, limit, data)
}

pub fn png_get(c: PixelCanvas) -> image::RgbImage {
    let w = c.width;
    let h = c.height;
    let mut imgbuf = image::ImageBuffer::new(w, h);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let col = c.data[y as usize * w as usize + x as usize];
        let r64: f64 = if col.red > 1.0 {
            1.0
        } else {
            if col.red < 0.0 {
                0.0
            } else {
                col.red
            }
        };
        let g64: f64 = if col.green > 1.0 {
            1.0
        } else {
            if col.green < 0.0 {
                0.0
            } else {
                col.green
            }
        };
        let b64: f64 = if col.blue > 1.0 {
            1.0
        } else {
            if col.blue < 0.0 {
                0.0
            } else {
                col.blue
            }
        };
        *pixel = image::Rgb([
            (r64 * 255.0).floor() as u8,
            (b64 * 255.0).floor() as u8,
            (b64 * 255.0).floor() as u8,
        ]);
    }
    imgbuf
}

fn str_row_get(row: u32, c: &PixelCanvas) -> String {
    let mut this_row: String = String::from("");
    for col in 0..c.width {
        let color = c.data[((row * c.width) + col) as usize];
        let color_str = str_from_color_get(color);
        this_row = format!("{}{}", this_row, color_str);
    }
    this_row
}

fn str_from_canvas_data_get(c: PixelCanvas) -> String {
    let max_cols: u32 = 70;
    let h = c.height;
    let mut data_string: String = String::from("");

    for row in 0..h {
        let mut this_row = str_row_get(row, &c);

        // split row if too long (multiple times if needed)
        let mut last_space_index: usize;
        let end: u32 = max_cols;
        let mut this_row_truncated: String;
        while this_row.len() > max_cols as usize {
            this_row_truncated = this_row.chars().take(*&end as usize).collect();

            //get actual_end
            if this_row_truncated.chars().last().unwrap() != ' ' {
                last_space_index = match this_row_truncated.rfind(' ') {
                    None => *&end as usize,
                    Some(x) => x,
                };
                this_row_truncated = this_row.chars().take(last_space_index).collect();
            } else {
                last_space_index = (max_cols - 1) as usize;
                this_row_truncated = this_row.chars().take(last_space_index).collect();
            }

            //add to main string
            data_string = format!("{}{}\n", data_string, this_row_truncated);

            //reduce to the remainder of the row
            this_row = this_row
                .chars()
                .skip(last_space_index + 1)
                .take(this_row.len())
                .collect();
        }
        data_string = format!("{}{}\n", data_string, str_remove_trailing_space(this_row));
    }
    data_string
}

pub fn str_from_color_get(col: tuples::Color) -> String {
    let color_clamped_to_zero_to_one = color_clamp(col);
    let r = (color_clamped_to_zero_to_one.red * CLAMP_LIMIT as f64) as u32;
    let g = (color_clamped_to_zero_to_one.green * CLAMP_LIMIT as f64) as u32;
    let b = (color_clamped_to_zero_to_one.blue * CLAMP_LIMIT as f64) as u32;
    format!("{} {} {} ", r, g, b)
}

fn color_clamp(mut col: tuples::Color) -> tuples::Color {
    if col.red < 0.0 {
        col.red = 0.0;
    }
    if col.green < 0.0 {
        col.green = 0.0;
    }
    if col.blue < 0.0 {
        col.blue = 0.0;
    }
    if col.red > 1.0 {
        col.red = 1.0;
    }
    if col.green > 1.0 {
        col.green = 1.0;
    }
    if col.blue > 1.0 {
        col.blue = 1.0;
    }
    col
}

fn str_remove_trailing_space(mut s: String) -> String {
    if s.len() > 0 && s.chars().last().unwrap() == ' ' {
        s.truncate(s.len() - 1);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_canvas() {
        //Creating a pixelCanvas
        let c = tuples::color(1.0, 2.0, 3.0);
        let pc = pixel_canvas(10, 20, c);
        assert_eq!(pc.width, 10);
        assert_eq!(pc.height, 20);
        assert_eq!(pc.length, 200);
        assert_eq!(tuples::get_bool_colors_are_equal(&pc.data[11], &c), true)
    }

    #[test]
    fn test_pixel_write() {
        //Writing a pixel to pixelCanvas
        let black = tuples::color(0.0, 0.0, 0.0);
        let mut pc = pixel_canvas(10, 20, black);
        assert_eq!(
            tuples::get_bool_colors_are_equal(&pc.data[32], &black),
            true
        );

        let red = tuples::color(1.0, 0.0, 0.0);
        pc = pixel_write(pc, 2, 3, red);
        assert_eq!(tuples::get_bool_colors_are_equal(&pc.data[32], &red), true)
    }

    #[test]
    fn test_str_from_color_get() {
        //getString_fromColor - returns clamped color string of 3 numbers, separated and ending with a space
        let c1 = tuples::color(1.5, 0.0, 0.0);
        let c2 = tuples::color(0.0, 0.5, 0.0);
        let c3 = tuples::color(-0.5, 0.0, 1.0);
        assert_eq!(str_from_color_get(c1), "255 0 0 ");
        assert_eq!(str_from_color_get(c2), "0 127 0 ");
        assert_eq!(str_from_color_get(c3), "0 0 255 ")
    }

    #[test]
    fn test_str_remove_trailing_space() {
        //getString_removeTrailingSpace - removes space if there is one
        let str1 = String::from("This is a test ");
        let str2 = String::from("This is a test");
        assert_eq!(str_remove_trailing_space(str1), str2)
    }

    #[test]
    fn test_str_remove_trailing_space_no_effect() {
        //getString_fromColor - returns clamped color string of 3 numbers, separated and ending with a space
        let str1 = String::from("This is a test");
        let str2 = String::from("This is a test");
        assert_eq!(str_remove_trailing_space(str1), str2)
    }

    #[test]
    fn test_ppm_get_header() {
        //Constructing the PPM header
        let black = tuples::color(0.0, 0.0, 0.0);
        let c = pixel_canvas(5, 3, black);
        let mut ppm = ppm_get(c);
        ppm.truncate(11);
        assert_eq!(ppm, "P3\n5 3\n255\n")
    }

    #[test]
    fn test_ppm_get_tail() {
        //Constructing the PPM header
        let black = tuples::color(0.0, 0.0, 0.0);
        let c = pixel_canvas(5, 3, black);
        let ppm = ppm_get(c);
        assert_eq!(ppm.chars().last().unwrap(), '\n')
    }

    #[test]
    fn test_str_from_canvas_data_get() {
        //Constructing the PPM pixel data
        let black = tuples::color(0.0, 0.0, 0.0);
        let mut pc = pixel_canvas(5, 3, black);
        let c1 = tuples::color(1.5, 0.0, 0.0);
        let c2 = tuples::color(0.0, 0.5, 0.0);
        let c3 = tuples::color(-0.5, 0.0, 1.0);
        pc = pixel_write(pc, 0, 0, c1);
        pc = pixel_write(pc, 2, 1, c2);
        pc = pixel_write(pc, 4, 2, c3);
        let ppm = ppm_get(pc);
        let header_size = 11;
        let just_data: String = ppm
            .chars()
            .skip(header_size)
            .take(ppm.len() - header_size)
            .collect();
        assert_eq!(just_data, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n")
    }

    #[test]
    fn test_str_from_canvas_data_get_long_lines_splitting() {
        //Splitting long lines in PPM files
        let black = tuples::color(1.0, 0.8, 0.6);
        let pc = pixel_canvas(10, 2, black);
        let ppm = ppm_get(pc);
        let header_size = 12;
        let just_data: String = ppm
            .chars()
            .skip(header_size)
            .take(ppm.len() - header_size)
            .collect();
        assert_eq!(just_data, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n")
    }
}
