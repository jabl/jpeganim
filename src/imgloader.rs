// Load a bunch of images named 0.jpg 1.jpg 2.jpg etc. into a vec

extern crate image;
use std::path::Path;

//use image::*;

fn img_load(dir: &Path) -> Vec<image::DynamicImage> {

    let mut id = 0;
    let mut v: Vec<image::DynamicImage> = vec![];
    loop {
        let imgpath = dir.join(id.to_string() + ".jpg");
        let img = match image::open(&imgpath) {
            Ok(val) => val,
            Err(_) => break,
        };
        v.push(img);
        id += 1;
    }
    v
}
