// Just messing around with knowledge from the variables chapter

// Different ways to define integers
const AGE_DECIMAL: u16 = 10_030 - 10_000;
const AGE_HEX: u8 = 0x1E;
const AGE_OCT: u8 = 0o36;
const AGE_BINARY: u8 = 0b11110;
const AGE_BYTE: u8 = b'F' - b'('; // ASCII character value as u8 int

// Integer overflow. In debug compiler release should fail
// In release build still fails
// const OVERFLOW: u8 = 59 + 200;

/*
  A character is 4 bytes so fits unicode
  char with '', strings with ""
*/
const EMOJI: char = '🤠';

// Semicolon at the end of line --> statement
// No semicolon at the end of line --> expression
fn with_return_value(x: i32) -> i32 {
    let y: i32 = 90;
    x + y
}

fn print_age() {
    println!("{AGE_DECIMAL}");
    println!("{AGE_HEX}");
    println!("{AGE_OCT}");
    println!("{AGE_BINARY}");
    println!("{AGE_BYTE}");
    // Casting u8 to u16 with into()
    println!(
        "{AGE_DECIMAL} == {AGE_HEX} -> {}",
        AGE_DECIMAL == AGE_HEX.into()
    );
}

fn handling_int_overflows() {
    // This also shows type suffixing
    // Useful for values w/o a variable

    // Wrapping add overflows
    println!("{}", 255u8.wrapping_add(5));

    // Checked add returns Option<u8>
    let num_ex2: u8 = 255;
    let sum = match num_ex2.checked_add(5) {
        Some(sum) => sum,
        None => {
            println!("Overflow :(");
            1
        }
    };
    println!("sum: {sum}");

    // Overflowing add like wrapping but tuple with overflow condition
    // Variable shadowing here as well
    let (sum, overflow) = 255u8.overflowing_add(5);
    if overflow {
        println!("Sad days. Overflow again!");
    }
    println!("sum next: {sum}");

    // Saturating add stops at max;
    println!("{}", 255u8.saturating_add(5));
    // Saturating sub stops at min
    println!("{}", 10u8.saturating_sub(25));
}

fn main() {
    print_age();
    println!("{EMOJI}");
    handling_int_overflows();

    // Different ways to use values in a tuple
    let tup: (i8, i32) = (-4, 4000);

    let (x1, y1) = tup;
    let x2 = tup.0;
    let y2 = tup.1;

    println!("{}, {}, {}", x1 + x2, y1 + y2, i32::from(x1) + y1);

    let res = with_return_value(55);
    println!("The function returned: {res}");

    let mut counter = 0;

    fn example() {
        let mut counter = 0;
        while counter < 5 {
            if counter == 3 {
                return;
            }
            println!("Counter: {}", counter + 1);
            counter += 1
        }
    }

    example();

    // Loop labels
    'outer_loop: loop {
        while counter < 10 {
            println!("Counter2: {}", counter + 1);

            if counter == 7 {
                println!("Breaking out of loop!");
                break 'outer_loop;
            }

            if counter == 8 {
                break;
            }

            counter += 1
        }
    }

    // Arrays (fixed length in stack)
    // Different for loop setups
    let arr = [0; 5];
    let arr2 = [1, 2, 3, 4, 5];

    for el in arr2 {
        println!("{el}");
    }

    // el not in scope
    // println!("el: {el}");

    for el in 10..15 {
        println!("{el}");
    }

    // Enumerations
    for (num, idx) in arr.iter().enumerate() {
        println!("arr: number {num} at index {idx}");
    }
}
