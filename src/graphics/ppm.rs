use crate::graphics::canvas::Canvas;

fn header(width: usize, height: usize) -> String {
    format!("P3\n{} {}\n255", width, height)
}
fn body(canvas: Canvas) -> String {
    let mut pixels: Vec<String> = vec!["255 255 255".to_string(); canvas.width * canvas.height];

    // scale colors to 255 and return it in a single Vec
    // ["255 0 0", "0 1 4", ...]
    canvas.grid.iter().enumerate().for_each(|(x, colors)| {
        colors.iter().enumerate().for_each(|(y, color)| {
            let scaled = color.scale(0.0, 255.0).round();

            pixels[x * canvas.width + y] =
                format!("{} {} {}", scaled.red, scaled.green, scaled.blue);
        });
    });

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
