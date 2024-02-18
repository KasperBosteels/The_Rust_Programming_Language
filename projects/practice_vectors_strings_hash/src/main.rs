use std::collections::HashMap;

fn main() {


}

fn vector_median_mode() {
    let mut v: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,0,0];
    v.sort();

    println!("{:?}", v);
    println!("{x}", x =&v[v.len() / 2]);
    
    let mut hsmp = HashMap::new();

    for v_value in &v {
        let count = hsmp.entry(*v_value).or_insert(0);
        *count += 1;
    }
    println!("{:?}", hsmp);
}

