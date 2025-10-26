use std::fmt::Debug;

pub fn cartesian_product<const K:usize, T:Copy+Ord+Debug>(iters:[Vec<T>;K]) -> Vec<Vec<T>> {
    let iter_lens = iters.clone().into_iter().map(|vec| vec.len()).collect::<Vec<usize>>();
    //let num_combinations:usize = iter_lens.iter().product();

    let mut output = vec![];
    let mut v1 = vec![];
    let mut v_len = 0;
    for (iter_idx, iter) in iters.clone().into_iter().enumerate() {
        println!("iter {:?}, iter_idx {}", iter, iter_idx);
        if iter_idx == 0 {
            println!("a");
            for val in iter {
                output.push(vec![val]);
            }
        } else {
            println!("b");
            for val in iter {
                for vec in output.clone().into_iter() {
                    let mut v2 = vec.clone();
                    v2.push(val);
                    println!("vec {:?}, v2 {:?}", vec, v2);
                    v1.push(v2);
                }
            }
            output = v1.clone()[v_len..v1.len()].to_vec();
        }
        println!("iter idx {}", iter_idx);
        v_len = v1.len();
        println!("v1 {:?}", v1);
        println!("output {:?}", output);
    }
    println!("output unsorted {:?}", output);
    output.sort();
    println!("output sorted {:?}", output);
    //panic!();
    output
}