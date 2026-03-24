use std::collections::HashMap;

const NUMBERS: [i32; 12] = [
    1, 25, 22, 9, 22, -138, 999, -111, 22, -10, 1, -12
];

fn main() {
    let mut nums = Vec::from(NUMBERS);
    let mut map = HashMap::new();

    nums.sort();
    println!("mediana = {}", nums[nums.len()/2]);

    for num in nums{
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

    let mut hash_vec: Vec<(&i32, &i32)> = map.iter().collect();
    hash_vec.sort_by(|a,b| b.1.cmp(a.1));

    println!("Sorted by value: {:?}", hash_vec);
    println!("Mode: {}", hash_vec[0].0);
}
