use std::fs::File;
use std::io::Write;

use matrices::{Matrix2d, Vector};

use Vector as Point;
use Vector as Colour;

//#[derive(Debug)]
//struct Point {
//    x:f32,
//    y:f32,
//    z:f32
//}
//
//#[derive(Debug)]
//struct Colour {
//    r:f32,
//    g:f32,
//    b:f32
//}

pub fn hello_ppm() {

    let a = Point::from_1darray([1.0, 2.0, 3.0]);
    let b = Point::from_1darray([1.0, 2.0, 3.0]);
    let c = a+b;
    println!("{:?}", c);

    let img_width  = 256;
    let img_height = 256;
    let height_order = f32::log10(img_height as f32) as u32 as usize + 1;
    //println!("{}, {}, {}, {}", f32::log10(img_height as f32), f32::log10(img_height as f32) as u32, f32::log10(img_height as f32) as u32 as usize, f32::log10(img_height as f32) as u32 as usize +1);

    let mut file = std::fs::File::create("ray_tracer_images/test.ppm").unwrap();
    writeln!(&mut file, "P3");
    writeln!(&mut file, "{} {}", img_width, img_height);
    writeln!(&mut file, "{}", 255);

    for i in 0..img_height {
        let lines_left = format!("{:>height_order$} lines left to write\r", img_height-i);
        //let c = b.as_str();
        //let a = vec![c, " lines left to write", "\r"].join("").as_bytes();
        std::io::stdout().write(lines_left.as_bytes());
        std::io::stdout().flush();
        for j in 0..img_width {
            let r: f32 = j as f32 / (img_width as f32 -1.0);
            let g: f32 = i as f32 / (img_width as f32 -1.0);
            let b: f32 = (i+j) as f32 / 256.0 / 1.35;

            //println!("{}, {}, {}", r, g, b);

            let ir = 256.0 * r;
            let ig = 256.0 * g;
            let ib = 256.0 * b;

            writeln!(&mut file, "{} {} {}", ir, ig, ib);
        }
    }   
}