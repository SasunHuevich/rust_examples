use std::time::{Duration, Instant};
use std::thread::sleep;


fn count_digits(value: u32, digit: u32) -> u32 {
    let mut count = 0;
    let mut tens = 1;
    let mut value = value;
    
    while tens <= value {
        let next_tens = tens * 10;
        let higher = value / next_tens;
        let current = (value / tens) % 10;
        let lower = value % tens;
        
        count += higher * tens;
        
        if current > digit {
            count += tens;
        } else if current == digit {
            count += lower + 1;
        }
        
        tens = next_tens;
    }
    
    count
}

fn main() {
    let now = Instant::now();

    let min: u32 = 400;
    let max: u32 = 1000000;
    let digit = 4;

    let result = count_digits(max, digit) - count_digits(min - 1, digit);
    println!("result: {}", result);

    let elapsed_time = now.elapsed();
    println!("Running slow_function() took {} microseconds.", elapsed_time.as_micros());

}

