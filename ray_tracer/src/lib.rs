use std::io::{Error, Write};

//use numeracy::matrices::Matrix;
use numeracy::matrices2::Matrix;
use atmospheric::image_processing::PPM;
use atmospheric::enums::PPMType;


//pub fn hello_ppm() -> PPM {
//
//    let ppm_type = PPMType::P3;
//    let img_width  = 256;
//    let img_height = 256;
//
//    let mut data = Matrix::null([3, img_width, img_height]);
//
//    for height_pos in 0..img_height {
//        for width_pos in 0..img_width {
//
//            let r: f32 = width_pos as f32 / (img_width as f32 -1.0);
//            let g: f32 = height_pos as f32 / (img_width as f32 -1.0);
//            let b: f32 = (height_pos + width_pos) as f32 / 256.0 / 1.35;
//
//            let ir = (256.0 * r) as u8;
//            let ig = (256.0 * g) as u8;
//            let ib = (256.0 * b) as u8;
//
//            //println!("hello ppm {:?}", (ir, ig, ib));
//
//            data[[0, width_pos, height_pos]] = ir;
//            data[[1, width_pos, height_pos]] = ig;
//            data[[2, width_pos, height_pos]] = ib;        }
//    }
//
//    let ppm = PPM {
//        type_: ppm_type, width:img_width, height:img_height,
//        max_colour_val:255, data
//    };
//
//    ppm
//}
//
///// path should include both relative path (directory), as well as filename
///// AND, filename MUST include the .ppm extension
//pub fn ppm_to_file(path:&str, ppm:PPM) -> Result<(), Error> {
//    let (img_width, img_height) = (ppm.width, ppm.height);
//    let height_order = f32::log10(img_height as f32) as u32 as usize + 1;
//
//    let mut file = std::fs::File::create(path).unwrap();
//    match ppm.type_ {
//        PPMType::P3 => writeln!(&mut file, "P3")?
//    }
//    writeln!(&mut file, "{} {}", ppm.width, ppm.height)?;
//    writeln!(&mut file, "{}", ppm.max_colour_val)?;
//
//    for height_pos in 0..img_height {
//        let lines_left = format!("{:>height_order$} lines left to write\r", img_height-height_pos);
//        //let c = b.as_str();
//        //let a = vec![c, " lines left to write", "\r"].join("").as_bytes();
//        std::io::stdout().write(lines_left.as_bytes())?;
//        std::io::stdout().flush()?;
//        for width_pos in 0..img_width {
//
//            let rgb_vec = ppm.data.get_submatrix([(0..3), width_pos..width_pos+1, height_pos..height_pos+1]).unwrap().array;
//            let (r, g, b) = (rgb_vec[0], rgb_vec[1], rgb_vec[2]);
//            writeln!(&mut file, "{} {} {}", r, g, b)?;
//
//            //let r: f32 = width_pos as f32 / (img_width as f32 -1.0);
//            //let g: f32 = height_pos as f32 / (img_width as f32 -1.0);
//            //let b: f32 = (height_pos + width_pos) as f32 / 256.0 / 1.35;
////
//            ////println!("{}, {}, {}", r, g, b);
////
//            //let ir = 256.0 * r;
//            //let ig = 256.0 * g;
//            //let ib = 256.0 * b;
////
//            //writeln!(&mut file, "{} {} {}", ir, ig, ib)?;
//        }
//    }
//    Ok(())
//
//}