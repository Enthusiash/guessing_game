fn main() {

    let days = [
        "First Day",
        "Second Day",
        "Third Day",
        "Fourth Day",
        "Fifth Day",
        "Sixth Day",
        "Seventh Day",
        "Eight Day",
        "Nineth Day",
        "Tenth Day",
        "Eleventh Day",
        "Twelveth Day",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming",
    ];
    for day in 0..12 {
        christmas_song(day, days, gifts);
        print!("\n\n");
    }
    
}

fn christmas_song(n: u32, days: [&str;12], gifts: [&str;12]) {
    for day in (0..n+1).rev() {
        if day == n {
            println!("On the {} days of Christmast my true love sent to me.", days[day as usize]);
            println!("{}", gifts[day as usize]);
        }
        else if day == 0 {
            println!("And {}", gifts[day as usize].to_lowercase());
        }
        else {
            println!("{}", gifts[day as usize]);
        }
    }
}