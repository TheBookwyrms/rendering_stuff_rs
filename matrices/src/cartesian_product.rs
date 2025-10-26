/// create the cartesian product of N vectors
pub fn cartesian_product<const K:usize, T:Copy+Ord>(iters:[Vec<T>;K]) -> Vec<Vec<T>> {
    let mut output = vec![vec![]];
    let mut subvec_outer = vec![];
    let mut prior_vec_len = 0;
    for iter in iters {
        for val in iter {
            for vec in output.clone().into_iter() {
                let mut subvec_inner = vec;
                subvec_inner.push(val);
                subvec_outer.push(subvec_inner);
            }
        }
        output = subvec_outer[prior_vec_len..subvec_outer.len()].to_vec();
        prior_vec_len = subvec_outer.len();
    }
    output.sort();
    output
}