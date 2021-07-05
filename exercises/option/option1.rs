// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

use std::fmt;

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        None => println!("None"),
        Some(arg) => println!("Some: {}", arg)
    };
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 6] = [None;6];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }

    for (i, &x) in numbers.iter().enumerate() {
        print_number(x);
    }
}
