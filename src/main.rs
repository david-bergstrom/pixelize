extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn main() {
    let mut img = image::open(&Path::new("test.png")).unwrap();

    let (width, height) = img.dimensions();

    let filter_type = image::Nearest;

    img = img.resize(width / 4, height / 4, filter_type);
    img = img.resize(width, height, filter_type);

    let ref mut fout = File::create(&Path::new("test_out.png")).unwrap();
    let _ = img.save(fout, image::PNG).unwrap();
}
