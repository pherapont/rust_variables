const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth",
];

const PRESENTS: [&str; 12] = [
    "partridge in a pear tree", "turtle doves", "French hens",
    "calling birds", "gold rings", "geese a-laying",
    "swans a-swimming", "maids a-milking", "ladies dancing",
    "lords a-leaping", "pipers piping", "drummers drumming"
];

const NUMBERS: [&str; 12] = [
    "One", "Two", "Three", "Four", "Five", "Six",
    "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"
];

fn main() {
    let mut day = 1;
    while day <= 12 {
        println!("On the {} day of Christmas my true love sent to me", DAYS[day-1]);

        let mut pres_num = 1;
        while pres_num < day{
            println!("{} {}",NUMBERS[day - pres_num], PRESENTS[day - pres_num]);
            pres_num += 1;
        }

        let prefix = if day == 1 {"A"} else {"And a"};
        println!("{prefix} {}.\n", PRESENTS[0]);

        day += 1;
    }
}
