/// sources:
/// https://web.archive.org/web/20121101013705/http://www.64lines.com/gif-width-height
/// https://giflib.sourceforge.net/whatsinagif/bits_and_bytes.html
/// letters are by ASCII character code https://ascii.cl/
pub fn get_gif_width_length<const N:usize>(data:&'static [u8; N]) -> Option<(u16, u16)> {
    let data_size = data.len();
    if data_size >= 10 && data[0] == 0x47/*G*/ && data[1] == 0x49/*I*/ && data[2] == 0x46/*F*/ {
        let height = data[7] as u16 * 256 + data[6] as u16;
        let width = data[9] as u16 * 256 + data[8] as u16;
        return Some((width, height))
    }
    None
}

// //Gets the GIF size from the array of data passed to the function
// static char get_gif_size(unsigned char* data, unsigned int data_size, unsigned short *width, unsigned short *height) {
//    //Check for valid GIF file (min 10 bytes for header and size, and the GIF signature)
//    if(data_size >= 10 && (data[0] == 'G' && data[1] == 'I' && data[2] == 'F')) {
//                *height = data[7]*256 + data[6];
//                *width = data[9]*256 + data[8];
//                return true;
//    } return false;                     //Not a valid GIF file
// }