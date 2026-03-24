const NUMBERS: [i32; 12] = [
    1, 25, -33, 257, 22, -138, 999, -111, 9, -10, 11, -12
];

fn main() {
    let mut nums = Vec::from(NUMBERS);
    nums.sort();
    for n in nums{
        println!("{n}");
    }
}
