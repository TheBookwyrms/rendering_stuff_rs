/// sources
/// https://web.archive.org/web/20121031042255/http://www.64lines.com/jpeg-width-height
/// letters are by ASCII character code https://ascii.cl/
pub fn get_jpeg_width_length_from_multiple<const N:usize>(data:&'static [u8; N]) -> Option<(u16, u16, Vec<u8>, bool)> {

    let mut possible_with_heights = vec![];

    println!("{:X?}", data[0..64].to_vec());
    let data_size = data.len();
    println!("{}", data_size);
    for j in 0..(data_size-4) {
        if data[j]==2 && data[j+1]==0 && data[j+2]==2 && data[j+3]==0 {
            println!("{:X?}", data[j-16..j+16].to_vec());
            println!("j = {}", j);
        }
    }

    println!("k");

    for k in 0..(data_size-1) {
        if data[k] == 0xFF && data[k+1] <= 0xCF && data[k+1] >= 0xC0 && data[k+1] != 0xC4 && data[k+1] != 0xCC {
            let height =data[k+5] as u16*256 + data[k+6] as u16;
            let width = data[k+7] as u16*256 + data[k+8] as u16;

            possible_with_heights.push((width, height));
            println!("k = {:>6}, height = {}, width = {}, where {:X?}", k, height, width, data[k-4..k+12].to_vec());
            //println!("k = {:>6}, k+1 = {}, where {:X?}", k, data[k+1], data[k-32..k+12].to_vec());
            //println!("{}", 0xC0);
        }
    }


    for l in 0..data_size {
        if data[l] == 0xFF && data[l+1] == 0xD8 {
            println!("ls {}", l);
        }
        if data[l] == 0xFF && data[l+1] == 0xD9 {
            println!("le {}", l);
        }
    }
    println!("ds {}", data_size);

    let pixel_data: Vec<u8> = data[7807..data_size-1].to_vec();
    let pixel_data: Vec<u8> = data[7807..data_size-1].to_vec();

    //panic!();

    if possible_with_heights.len() == 1 {
        return Some((possible_with_heights[0].0, possible_with_heights[0].1, pixel_data, false))
    } else if possible_with_heights.len() > 1 {
        return Some((possible_with_heights[possible_with_heights.len()-1].0, possible_with_heights[possible_with_heights.len()-1].1, pixel_data, true))
    } else {
        return None
    }

    //panic!();
    //let mut i = 0;
    //println!("{:x}, {:x}, {:x}, {:x}", data[i], data[i+1], data[i+2], data[i+3]);
    //if data[i] == 0xFF && data[i+1] == 0xD8 && data[i+2] == 0xFF && data[i+3] ==  0xE1 {//0xE0 {
    //    i += 4;
    //    if data[i+2] == 0x4A/*'J'*/ && data[i+3] == 0x46/*'F'*/ && data[i+4] == 0x49/*'I'*/ && data[i+5] == 0x46/*'F'*/ && data[i+6] == 0x00 {
    //        let mut block_length = (data[i] as u16 * 256 + data[i+1] as u16) as usize;
    //        while i<data_size {
    //            i += block_length;
    //            if i >= data_size {
    //                println!("a");
    //                return None
    //            } else if data[i] != 0xFF {
    //                println!("b");
    //                return None
    //            } else if data[i+1] == 0xC0  {
    //                let height = data[i+5] as u16*256 + data[i+6] as u16;
    //                let width  = data[i+7] as u16*256 + data[i+8] as u16;
    //                return Some((width, height))
    //            } else {
    //                i+=2;
    //                block_length = (data[i] as u16 * 256 + data[i+1] as u16) as usize;
    //            }
    //        }
    //                println!("c");
    //        None
    //    } else {
    //                println!("d");
    //        None
    //    }
    //} else {
    //                println!("e");
    //    None
    //}
}

// //Gets the JPEG size from the array of data passed to the function, file reference: http://www.obrador.com/essentialjpeg/headerinfo.htm
// static char get_jpeg_size(unsigned char* data, unsigned int data_size, unsigned short *width, unsigned short *height) {
//    //Check for valid JPEG image
//    int i=0;   // Keeps track of the position within the file
//    if(data[i] == 0xFF && data[i+1] == 0xD8 && data[i+2] == 0xFF && data[i+3] == 0xE0) {
//       i += 4;
//       // Check for valid JPEG header (null terminated JFIF)
//       if(data[i+2] == 'J' && data[i+3] == 'F' && data[i+4] == 'I' && data[i+5] == 'F' && data[i+6] == 0x00) {
//          //Retrieve the block length of the first block since the first block will not contain the size of file
//          unsigned short block_length = data[i] * 256 + data[i+1];
//          while(i<data_size) {
//             i+=block_length;               //Increase the file index to get to the next block
//             if(i >= data_size) return false;   //Check to protect against segmentation faults
//             if(data[i] != 0xFF) return false;   //Check that we are truly at the start of another block
//             if(data[i+1] == 0xC0) {            //0xFFC0 is the "Start of frame" marker which contains the file size
//                //The structure of the 0xFFC0 block is quite simple [0xFFC0][ushort length][uchar precision][ushort x][ushort y]
//                *height = data[i+5]*256 + data[i+6];
//                *width = data[i+7]*256 + data[i+8];
//                return true;
//             }
//             else
//             {
//                i+=2;                              //Skip the block marker
//                block_length = data[i] * 256 + data[i+1];   //Go to the next block
//             }
//          }
//          return false;                     //If this point is reached then no size was found
//       }else{ return false; }                  //Not a valid JFIF string
//          
//    }else{ return false; }                     //Not a valid SOI header
// }