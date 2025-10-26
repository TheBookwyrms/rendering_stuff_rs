pub fn cartesian_product<const K:usize, T:Copy+Ord>(iters:[Vec<T>;K]) -> Vec<Vec<T>> {

    let mut output = vec![];
    let mut v1 = vec![];
    let mut v_len = 0;
    for (iter_idx, iter) in iters.clone().into_iter().enumerate() {
        if iter_idx == 0 {
            for val in iter {
                output.push(vec![val]);
            }
        } else {
            for val in iter {
                for vec in output.clone().into_iter() {
                    let mut v2 = vec.clone();
                    v2.push(val);
                    v1.push(v2);
                }
            }
            output = v1.clone()[v_len..v1.len()].to_vec();
        }
        v_len = v1.len();
    }
    output.sort();
    output
}