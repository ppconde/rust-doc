// write the twelve days of christmas carol lyrics

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "A partridge in a pear tree.\n",
    "Two turtle doves,\n",
    "Three french hens\n",
    "Four calling birds\n",
    "Five golden rings\n",
    "Six geese a-laying\n",
    "Seven swans a-swimming,\n",
    "Eight maids a-milking\n",
    "Nine ladies dancing\n",
    "Ten lords a-leaping\n",
    "Eleven pipers piping\n",
    "Twelve drummers drumming\n",
];

fn main() {
    for i in 0..12 {
        println!("{}", christmas_carol(i));
    }
}

fn christmas_carol(day: u32) -> String {
    let mut lyrics = String::new();
    lyrics.push_str(&format!(
        "On the {} day of Christmas\nmy true love sent to me\n",
        DAYS[day as usize]
    ));

    for i in (0..day + 1).rev() {
        if i == 0 {
            lyrics.push_str("And a partridge in a pear tree\n");
        } else {
            lyrics.push_str(&format!("{} ", GIFTS[i as usize]));
        }
    }

    lyrics
}
