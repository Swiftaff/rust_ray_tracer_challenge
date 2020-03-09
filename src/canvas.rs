//use std::f64;

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
/*
fn pixel_get(canvas: PixelCanvas, x: u32, y: u32) -> tuples::Color {
    let index = canvas.width * y + x;
    if index >= 0 && index < canvas.length {
        canvas.data[index];
    }
    canvas
}

pub fn ppm_get(c: PixelCanvas) -> String {
    let w = c.width.to_string();
    let h = c.height.to_string();
    let limit = "255";
    let data = str_from_canvas_data_get(c, 255);
    "P3\n" + w + " " + h + "\n" + limit + "\n" + data
}

fn str_from_canvas_data_get(c: PixelCanvas, clampLimit:u32) {
    let mut colorStringArray = [];
    let mut rowArray = [];
    let mut data = Vec::with_capacity(length as usize);
    for _i in 0..length {
        data.push(default_color);
    }

    c.data.map(col => colorStringArray.push(getString_fromColor(col, clampLimit)));
    for (let rowStartIndex = 0; rowStartIndex < c.width * c.height; rowStartIndex += c.width) {
        let thisRow = "";
        for (let colIndex = 0; colIndex < c.width; colIndex++) {
            thisRow += colorStringArray[rowStartIndex + colIndex];
        }
        if (thisRow.length > 70) {
            let lastSpaceIndex = thisRow.substring(0, 70);
            if (thisRow.charAt(70) !== " ") {
                lastSpaceIndex = thisRow.substring(0, 70).lastIndexOf(" ");
            }
            let nextRowOverflow = thisRow.substring(lastSpaceIndex + 1);
            thisRow = thisRow.substring(0, lastSpaceIndex);
            rowArray.push(getString_removeTrailingSpace(thisRow));

            rowArray.push(getString_removeTrailingSpace(nextRowOverflow));
        } else {
            rowArray.push(getString_removeTrailingSpace(thisRow));
        }
    }
    return rowArray.join("\n") + "\n";
}
*/
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
}
