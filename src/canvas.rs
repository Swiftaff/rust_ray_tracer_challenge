use std::f64;

mod tuples;

#[derive(Debug)]
pub struct Pixel_canvas {
    pub data: [tuples::Color],
    pub width: u32,
    pub height: u32
}

pub fn pixel_canvas(width:u32,height:u32, default_color:Color) -> Pixel_canvas {
    let data [u32; width * height] = [default_color; width * height];
    let length = width * height;
    { data:data, width:width, height:height, length:length }
}

pub fn pixel_write(canvas: tuples::Pixel_canvas, x: u32, y: u32, col: tuples::Color) -> Pixel_canvas {
    let index = (c.width * y + x) as u32;
    canvas.data[index] = col;
    canvas
}

fn pixel_get(canvas: Pixel_canvas, x: u32, y: u32) -> tuples::Color {
    let index = canvas.width * y + x;
    if index >= 0 && index < canvas.length {
        canvas.data[index];
    }
    canvas
}

pub fn ppm_get(c: Pixel_canvas) -> String {
    let w = c.width.to_string();
    let h = c.height.to_string();
    let limit = "255";
    let data = str_from_canvas_data_get(c, 255);
    "P3\n" + w + " " + h + "\n" + limit + "\n" + data
}

fn str_from_canvas_data_get(c: Pixel_canvas, clampLimit:u32) {
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