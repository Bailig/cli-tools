// integers has signed and unsigned variants
// signed: i8, i16, i32, i64, i128, isize
// unsigned: u8, u16, u32, u64, u128, usize
// integer literals can be specified in following ways:
// decimal: 98_222
// hexadecimal: 0xff
// octal: 0o77
// binary: 0b1111_0000
// byte (u8 only): b'A'

// floating point numbers: f32, f64

// boolean: bool

// character: char
// one char represents one unicode scalar value
// a unicode scalar value
// a char is 4 bytes in size (32 bits)
// chars are UCS-4 or UTF-32 encoded
// char literals are specified with single quotes
// strings are UTF-8 encoded so strings do not use chars internally

// tuple with more than 12 elements will have limited functionality
// array with more than 32 elements will have limited functionality

use std::f32::consts::PI;

pub fn run() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_array = [coords.0, coords.1];
    print_array(coords_array);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], PI, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

fn print_distance((x, y): (f32, f32)) {
    println!(
        "Distance to the origin is {}",
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    );
}
