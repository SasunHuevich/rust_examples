fn main() {
    println!("33.0 fahrenherit = {} celsius\n", fahrenheit_to_celsius(33.0));
    println!("9 fibonacci = {}\n", fibonacci(9));
    twelwe_days_of_xmas();
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

fn fibonacci(value: i32) -> i32 {
    if value == 0 {
        0
    } else if value == 1 {
        1
    } else {
        fibonacci(value - 1) + fibonacci(value - 2)
    }
}


fn twelwe_days_of_xmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds",
        "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", 
        "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"
    ];

    println!("On the first day of Christmas,");
    println!("my true love sent to me");
    println!("A partridge in a pear tree.\n");
    
    for number in 1..12 {
        println!("On the {} day of Christmas,", days[number]);
        println!("my true love sent to me");

        for number2 in (1..number).rev() {
            println!("{},", gifts[number2])
        }

        println!("And a partridge in a pear tree.\n");
    }
}