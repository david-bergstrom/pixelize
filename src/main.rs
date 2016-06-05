extern crate image;
extern crate getopts;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::env;

use image::GenericImage;
use getopts::Options;

fn pixelize_image(img: &image::DynamicImage) -> image::DynamicImage {
    let (width, height) = img.dimensions();
    let filter_type = image::Nearest;

    img.resize(width / 4, height / 4, filter_type)
        .resize(width, height, filter_type)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} IMAGE -o OUTPUT_IMAGE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    // TODO: Add factor

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = match matches.opt_str("o") {
        Some(x) => x,
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut img = match image::open(&Path::new(&input)) {
        Ok(img) => img,
        Err(error) => {
            println!("Could not open file \"{}\": {}", input, error.description());
            return;
        }
    };

    img = pixelize_image(&img);

    // TODO: Handle output errors
    let ref mut fout = File::create(&Path::new(&output)).unwrap();
    let _ = img.save(fout, image::PNG).unwrap();
}
