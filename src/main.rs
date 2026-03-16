use std::io;

fn main() {
    println!("Convert tmperature from Farenhait to Celsius.");
    println!("Input temperature in Celsius: ");

    let mut tmp = String::new();

    io::stdin()
        .read_line(&mut tmp)
        .expect("Wrong input!");

    let tmp: i32 = tmp.trim().parse().expect("Type a number.");

    let tmp_c = convert_to_celsius(tmp);
    println!("{tmp_c}");
}

fn convert_to_celsius (tmp: i32) -> i32 {
    (tmp - 32) * 5 / 9
}
