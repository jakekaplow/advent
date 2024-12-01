use std::fs;
use std::env;
use std::collections::HashMap;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("error reading input");

    let v: Vec<u64> = contents
        .split_whitespace()
        .map(|s| s.parse::<u64>()
            .unwrap())
        .collect();
    
    let mut l1: Vec<u64> = Vec::new();
    let mut l2: Vec<u64> = Vec::new();

    for i in 0..v.len() {
        if (i % 2) == 0 {
            l1.push(v[i]);
        } else {
            l2.push(v[i]);
        }
    }

    l1.sort();
    l2.sort();

    let mut diffs: Vec<u64> = Vec::new();

    for i in 0..l1.len() {
        diffs.push(l1[i].abs_diff(l2[i]));
    }

    let sum: u64 = diffs.iter().sum();
    println!("{}", sum);
    println!("***");

    let mut l2_count: HashMap<u64, u64> = HashMap::new();
    for v2 in l2 {
        let _ = *l2_count.entry(v2)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    let mut ss: u64 = 0;

    for v1 in l1 {
        ss += l2_count.get(&v1).unwrap_or(&0) * v1;
    }

    println!("{}", ss);
}
