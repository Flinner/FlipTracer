use crate::features::{
    canvas::Canvas,
    colors::{self, Color},
};

fn header(width: usize, height: usize) -> String {
    format!("P3\n{} {}\n255", width, height)
}
fn body(canvas: Canvas) -> String {
    let mut pixels: Vec<String> = vec![];

    // scale colors to 255 and return it in a single Vec
    // ["255 0 0", "0 1 4", ...]
    for colors in canvas.grid.iter() {
        for color in colors.iter() {
            let scaled = color.scale(0.0, 255.0).round();

            pixels.push(format!("{} {} {}", scaled.red, scaled.green, scaled.blue));
        }
    }

    // split rows into elements of 5.
    // [
    // "255 0 0"x5,
    // "255 0 0"x5,...
    // ]
    let rows: Vec<String> = pixels.chunks(5).map(|row| row.join(" ")).collect();

    rows.join("\n")
}
/// returns ppm image encoded in a `String`
pub fn new(canvas: Canvas) -> String {
    let header = header(canvas.width, canvas.height);
    let body = body(canvas);
    format!("{}\n{}\n", header, body)
}
