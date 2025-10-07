use std::env;

fn linear_index_to_indices<const K:usize>(linear_index:usize, shape:[usize;K]) -> [usize; K] {

    let mut indices = shape.clone();

    let mut curr_max :usize = shape.iter().product();
    let mut curr_lin_idx = linear_index.clone();

    for (i, s_size) in shape.iter().enumerate().rev() {
        // IMPORTANT!!!
        // The divisions here truncate the values
        // not a pure division with exact values
        // ex: 3.75 is truncated to 3
        let section_len = curr_max/s_size;
        let section = curr_lin_idx/section_len;
        curr_lin_idx -= section*section_len;
        curr_max = curr_max/s_size;
        indices[i] = section;
        //indices[i] = s_size*curr_lin_idx/curr_max;
        //curr_lin_idx -= (s_size*curr_lin_idx/curr_max)*(curr_max/s_size);
        //curr_max = curr_max/s_size;
    }

    indices
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let _shape = [4, 2, 3, 4, 3, 25, 43];
    let shape = [4, 2, 3];
    
    //let linear_index = 2;
    //let mut indices = shape.clone();
    
    //let mut curr_max :usize = shape.iter().product();
    //let mut curr_lin_idx = linear_index.clone();
    //println!("{}, {}", curr_max, curr_lin_idx);

    for linear_index in 0..shape.iter().product() {

        let indices = linear_index_to_indices(linear_index, shape.clone());

        //let mut curr_max :u32 = shape.iter().product();
        //let mut curr_lin_idx = linear_index.clone();
//
        //for (i, s_size) in shape.iter().enumerate().rev() {
        //    let section_len = curr_max/s_size;
        //    let section = (curr_lin_idx/section_len);
        //    curr_lin_idx -= section*section_len;
        //    curr_max = curr_max/s_size;
        //    indices[i] = section;
        //}

        println!("{:?}", indices);
    }

    // for (i, s_size) in shape.iter().enumerate().rev() {
    //     println!("i, s, {}, {}", i, s_size);
// 
    //     let section_len = curr_max/s_size;
    //     let relative_position = curr_lin_idx%section_len;
    //     let section = (curr_lin_idx/section_len);
// 
    //     println!("m {}, {}, {}, {}", curr_lin_idx, relative_position, section_len, section);
    //     //curr_lin_idx = curr_lin_idx - relative_position*section_len;
    //     curr_lin_idx -= section*section_len;
    //     curr_max = curr_max/s_size;
    //     indices[i] = section;
// 
    //     println!("{}, {}, {}, {}", section_len, relative_position, curr_lin_idx, curr_max);
    //     println!("");
// 
    // }
// 
    // println!("{:?}", indices);
}