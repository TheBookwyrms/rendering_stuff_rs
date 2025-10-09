fn reverse_array(array:Vec<usize>) -> Vec<usize> {
    let mut rev_arr= vec![0;array.len()];
    for (i, j) in array.into_iter().rev().enumerate() {
        rev_arr[i] = j;
    }
    rev_arr.to_vec()
}

fn linear_index_to_indices(linear_index:usize, shape:Vec<usize>) -> Vec<usize> {

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
    }
    indices
}


pub fn turn_indices_into_linear_index(shape:Vec<usize>, indices:Vec<usize>) -> usize {
    let ndims = shape.clone().len();
    let rev_ind = reverse_array(indices.clone());
    
    let mut linear_idx = 0;
    
    for i in (0..ndims).into_iter().rev() {
        let mut idx_max = 1;
        //for j in i..(ndims-1) {
        for j in 0..i {
            idx_max *= shape[j];
        }
        linear_idx += indices[i]*idx_max;
    }
    //println!("{}", linear_idx);
    linear_idx
}


fn main() {
    let shape = vec![2, 3, 3];
    for linear_index in 0..20 {
        let indices = linear_index_to_indices(linear_index, shape.clone());
        let new_linear_index = turn_indices_into_linear_index(shape.clone(), indices.clone());
        println!("{}, {:?}, {}", linear_index, indices, new_linear_index);
    }
}