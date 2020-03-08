//use std::f64;

use crate::tuples;

#[derive(Debug, Clone)]
pub struct PixelCanvas {
    pub width: u32,
    pub height: u32,
    pub length: u32,
    pub data: Vec<tuples::Color>
}

pub fn pixel_canvas(width:u32,height:u32, default_color:tuples::Color) -> PixelCanvas {
    let length = width * height;
    let mut data = Vec::with_capacity(length as usize);
    for _i in 0..length {
        data.push(default_color);
    }
    PixelCanvas { data:data, width:width, height:height, length:length }
}

pub fn pixel_write(canvas: PixelCanvas, x: u32, y: u32, col: tuples::Color) -> PixelCanvas {
    let index = (canvas.width * y + x) as usize;
    let mut new_canvas = canvas;
    new_canvas.data[index] = col;
    new_canvas
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
    let colorStringArray = [];
    let rowArray = [];
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

fn str_from_color_get(col: Pixel_color, clampLimit:u32) {
    let colorClampedToZeroToOne = color_clamp(col);
    let r =         (colorClampedToZeroToOne.red * clampLimit) as u32;
    return (
 +
        " " +
        (colorClampedToZeroToOne.green * clampLimit) +
        " " +
        (colorClampedToZeroToOne.blue * clampLimit) +
        " "
    );
}

fn color_clamp(col:Color) {
    if col.red < 0 {col.red = 0;}
    if col.green < 0 {col.green = 0;}
    if col.blue < 0 {col.blue = 0;}
    if col.red > 1 {col.red = 1;}
    if col.green > 1 {col.green = 1;}
    if col.blue > 1 {col.blue = 1;}
    col
}
*/

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_pixel_canvas() {
        //Creating a pixelCanvas
        let c = tuples::color(1.0,2.0,3.0);
        let pc =pixel_canvas(10,20,c);
        assert_eq!(pc.width, 10);
        assert_eq!(pc.height, 20);
        assert_eq!(pc.length, 200);
        assert_eq!(tuples::get_bool_colors_are_equal(&pc.data[11], &c),true)
    }

    #[test]
    fn test_pixel_write() {
        //Writing a pixel to pixelCanvas
        let black = tuples::color(0.0,0.0,0.0);
        let mut pc = pixel_canvas(10,20,black);
        assert_eq!(tuples::get_bool_colors_are_equal(&pc.data[32], &black),true);

        let red = tuples::color(1.0,0.0,0.0);
        pc = pixel_write(pc,2,3,red);
        assert_eq!(tuples::get_bool_colors_are_equal(&pc.data[32], &red),true)
    }

}