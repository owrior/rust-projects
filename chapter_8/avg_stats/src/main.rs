use std::collections::HashMap;

fn main() {
    let vector = vec![1, 2, 7, 7, 2, 2];
    dbg!(median(&vector));
    dbg!(mode(&vector));
    println!("{:?}", vector)
}

fn median(vector: &Vec<i32>) -> i32 {
    let mut int_vector = vector.to_vec();
    let length = int_vector.len();
    int_vector.sort();

    let midpoint = length / 2;
    if length % 2 == 0 {
        int_vector[midpoint]
    } else {
        (int_vector[midpoint] + int_vector[midpoint + 1]) / 2
    }
}

fn mode(vector: &Vec<i32>) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for item in vector {
        let count = counts.entry(*item).or_insert(0);
        *count += 1;
    }
    let max_count = *counts.iter().max_by_key(|a| a.1).unwrap().1;
    let mut modes = vec![];
    for (key, val) in counts {
        if val == max_count {
            modes.push(key);
        }
    }
    modes
}
